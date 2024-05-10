use clap::Parser;
use console::style;
use lang::schema::Schema;
use std::{fs, path::Path, time::Instant};

mod emit;
mod lang;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Options {
    /// Path to the input schema file
    #[clap(short, long)]
    input: String,

    /// Path to the output code file
    #[clap(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let input = fs::read_to_string(options.input).unwrap();

    let started = Instant::now();
    println!("{} Parsing schema...", style("[1/2]").bold().dim());

    let output_path = Path::new(&options.output);
    let output_file_extension = output_path.extension().unwrap().to_str().unwrap();
    let output_file_stem = output_path.file_stem().unwrap().to_str().unwrap();
    let strategy = Strategy::from_file_extension(output_file_extension).unwrap();

    let tokens = lang::lexer::get_tokens(&input);
    let schema = lang::parser::get_schema(tokens)?;

    println!(
        "{} Emitting {} code...",
        style("[2/2]").bold().dim(),
        style(format!("{:?}", strategy)).bold().bright()
    );
    let contents = strategy.emit_schema(output_file_stem, &schema);
    println!("done in {}μs", started.elapsed().as_micros());

    fs::write(options.output, contents).unwrap();

    Ok(())
}

#[derive(Debug, Clone)]
enum Strategy {
    RON,
    JSON,
    Rust,
    TypeScript,
    Dart,
}

impl Strategy {
    pub fn from_file_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Strategy::Rust,
            "ts" => Strategy::TypeScript,
            "dart" => Strategy::Dart,
            "ron" => Strategy::RON,
            "json" => Strategy::JSON,
            _ => return None,
        }
        .into()
    }

    fn emit_schema(&self, name: &str, schema: &Schema) -> String {
        match self {
            Strategy::Rust => emit::rs::emit_schema(name, &schema),
            Strategy::TypeScript => emit::ts::emit_schema(name, &schema),
            Strategy::Dart => emit::dart::emit_schema(name, &schema),
            Strategy::RON => emit::ron::emit_schema(name, &schema),
            Strategy::JSON => emit::json::emit_schema(name, &schema),
        }
    }
}
