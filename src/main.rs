use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, help="Lox source file path")]
    file_path: Option<String>,
}

impl Args {
    fn new() -> Result<Self, String> {
        let args = Self::parse();
            Ok(args)
    }
}

fn main() {
    match Args::new() {
        Ok(args) => {
            if let Some(file_path) = args.file_path {
                run_file(&file_path);
            } else {
                run_prompt();
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn run_file(file_path: &str) {
    println!("Running file: {}", file_path);
}

fn run_prompt() {
    println!("Running prompt");
}
