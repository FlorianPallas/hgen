use clap::Parser;
use console::style;
use lang::schema::Schema;
use std::{fmt::Display, fs, path::Path, time::Instant};

mod emit;
mod lang;

const FILE_EXTENSION: &str = "hgen";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Options {
    /// Path to the input schema file
    #[clap(short, long)]
    input: String,

    /// The type of code to emit
    #[clap(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let started = Instant::now();

    let input_path = Path::new(&options.input).to_path_buf();
    let input_dir = input_path.parent().unwrap();
    let input_file_name = input_path.file_stem().unwrap();

    let mut root = Schema::new();
    let mut sources = vec![input_path.clone()];

    println!("parsing schema");
    while let Some(path) = sources.pop() {
        println!("{}", style(path.display()).dim());

        let input = fs::read_to_string(&path).unwrap();
        let tokens = lang::lexer::get_tokens(&input);
        let schema = lang::parser::get_schema(tokens)?;

        sources.extend(
            schema
                .imports
                .iter()
                .map(|name| input_dir.join(name).with_extension(FILE_EXTENSION)),
        );

        root.extend(schema);
    }

    options.output.split(",").for_each(|output| {
        let strategy = Strategy::parse(output).expect("Unsupported output");

        println!("emitting {} code", style(&strategy).cyan().bold());

        let output_path = input_dir
            .join(input_file_name)
            .with_extension(strategy.extension());

        println!("{}", style(output_path.display()).dim());

        let output = strategy.emit(input_file_name.to_str().unwrap(), &root);
        fs::write(output_path, output).unwrap();
    });

    println!("done in {}μs", started.elapsed().as_micros());

    Ok(())
}

#[derive(Debug, Clone)]
enum Strategy {
    JSON,
    Rust,
    TypeScript,
    Dart,
}

impl Strategy {
    fn emit(&self, name: &str, schema: &Schema) -> String {
        match self {
            Strategy::Rust => emit::rs::emit_schema(name, &schema),
            Strategy::TypeScript => emit::ts::emit_schema(name, &schema),
            Strategy::Dart => emit::dart::emit_schema(name, &schema),
            Strategy::JSON => emit::json::emit_schema(name, &schema),
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        return Self::from_extension(value);
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Strategy::Rust,
            "ts" => Strategy::TypeScript,
            "dart" => Strategy::Dart,
            "json" => Strategy::JSON,
            _ => return None,
        }
        .into()
    }

    fn extension(&self) -> &'static str {
        match self {
            Strategy::Rust => "rs",
            Strategy::TypeScript => "ts",
            Strategy::Dart => "dart",
            Strategy::JSON => "json",
        }
    }
}

impl Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Strategy::Rust => write!(f, "Rust"),
            Strategy::TypeScript => write!(f, "TypeScript"),
            Strategy::Dart => write!(f, "Dart"),
            Strategy::JSON => write!(f, "JSON"),
        }
    }
}
