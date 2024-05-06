#![feature(test)]

extern crate test;

use clap::Parser;
use console::style;
use schema::*;
use std::{fs, str::FromStr, time::Instant};

mod emit;
mod parse;
mod schema;

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

    /// Explicitly specify the generation strategy to use, by default it is inferred from the output file extension
    #[clap(long)]
    strategy: Option<Strategy>,

    /// Include reflection metadata in the generated code
    #[clap(long, default_value_t = false)]
    reflection: bool,
}

#[derive(Debug, Clone)]
enum Strategy {
    Rust,
    TypeScript,
    Dart,
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "rust" => Ok(Strategy::Rust),
            ".rs" => Ok(Strategy::Rust),
            "typescript" => Ok(Strategy::TypeScript),
            ".ts" => Ok(Strategy::TypeScript),
            "dart" => Ok(Strategy::Dart),
            ".dart" => Ok(Strategy::Dart),
            _ => Err("Unsupported strategy".to_owned()),
        }
    }
}

fn main() {
    let options = Options::parse();

    let started = Instant::now();

    println!("{} Parsing schema...", style("[1/2]").bold().dim());
    let input = fs::read_to_string(options.input.clone()).unwrap();
    let schema = parse::parse_schema(&input);

    let strategy = options.strategy.unwrap_or(
        Strategy::from_str(&format!(".{}", options.output.split('.').last().unwrap())).unwrap(),
    );
    println!(
        "{} Emitting {} code...",
        style("[2/2]").bold().dim(),
        style(format!("{:?}", strategy)).bold().bright()
    );

    let contents = emit_schema(&schema, strategy);
    fs::write(options.output, contents).unwrap();

    println!("done in {}μs", started.elapsed().as_micros());
}

fn emit_schema(schema: &Schema, strategy: Strategy) -> String {
    match strategy {
        Strategy::Rust => emit::rs::emit_schema(&schema),
        Strategy::TypeScript => emit::ts::emit_schema(&schema),
        Strategy::Dart => emit::dart::emit_schema(&schema),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn benchmark_parser(b: &mut Bencher) {
        let input = fs::read_to_string("examples/duxtura/api.hgen").unwrap();

        b.iter(|| {
            parse::parse_schema(&input);
        });
    }

    #[bench]
    pub fn benchmark_rust(b: &mut Bencher) {
        let schema = parse::parse_schema(&fs::read_to_string("examples/duxtura/api.hgen").unwrap());

        b.iter(|| {
            emit::rs::emit_schema(&schema);
        });
    }

    #[bench]
    pub fn benchmark_typescript(b: &mut Bencher) {
        let schema = parse::parse_schema(&fs::read_to_string("examples/duxtura/api.hgen").unwrap());

        b.iter(|| {
            emit::ts::emit_schema(&schema);
        });
    }

    #[bench]
    pub fn benchmark_dart(b: &mut Bencher) {
        let schema = parse::parse_schema(&fs::read_to_string("examples/duxtura/api.hgen").unwrap());

        b.iter(|| {
            emit::dart::emit_schema(&schema);
        });
    }
}
