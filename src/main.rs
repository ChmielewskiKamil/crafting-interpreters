use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, help = "Lox source file path")]
    file_path: Option<PathBuf>,
}

impl Args {
    fn new() -> Result<Self, String> {
        let args = Self::parse();
        Ok(args)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::new()?;
    if let Some(file_path) = args.file_path {
        let content = read_file(&file_path)?;
        run_lexical_analysis(&content);
    } else {
        run_prompt();
    }
    Ok(())
}

fn read_file(file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn run_lexical_analysis(content: &str) {
    println!("Performing lexical analysis on content: {}", content);
}

fn run_prompt() {
    println!("Running prompt");
}
