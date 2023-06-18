use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, help="Lox source file path")]
    file_path: String,
}

impl Args {
    fn new() -> Result<Self, String> {
        let args = Self::parse();
        if args.file_path.is_empty() {
            Err("No file path provided".to_string())
        } else {
            Ok(args)
        }
    }
}

fn main() {
    Args::new().unwrap();
}
