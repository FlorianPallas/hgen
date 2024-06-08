use clap::Parser;
use console::style;
use lang::schema::Schema;
use std::{fmt::Display, fs, path::Path, time::Instant};

mod emit;
mod lang;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Options {
    /// Path to the input schema file
    #[clap(short, long)]
    input: String,

    /// Path to the output file
    #[clap(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let started = Instant::now();

    let input_path = Path::new(&options.input).to_path_buf();
    let input_file_name = input_path.file_stem().unwrap();

    let output_path = Path::new(&options.output).to_path_buf();
    let output_dir = output_path.parent().unwrap();
    let output_file_extension = output_path.extension().unwrap().to_str().unwrap();

    let strategy = Strategy::parse(output_file_extension).expect("Unsupported output");

    println!("parsing schema");
    println!("{}", style(input_path.display()).dim());

    let source = fs::read_to_string(&input_path)?;
    let schema = Schema::parse(&source);
    println!("{:?}", schema);
    let output = strategy.emit(input_file_name.to_str().unwrap(), &schema);

    println!("emitting {} code", style(&strategy).cyan().bold());
    println!("{}", style(output_path.display()).dim());

    fs::create_dir_all(output_dir).unwrap();
    fs::write(&output_path, output).unwrap();

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
            _ => panic!("unsupported strategy: {:?}", self),
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
