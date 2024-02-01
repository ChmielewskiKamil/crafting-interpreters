use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Read},
};

mod token;

struct Scanner {
    source_file: Vec<u8>,
    scanned_tokens: Vec<token::Token>,
    current_char: usize,
    current_line: usize,
    current_lexeme_start: usize,
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.current_char >= self.source_file.len()
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Too many arguments! Usage: sol-ast [path/to/file.sol]");
    }
    // If user passed in a path to a file
    else if args.len() == 2 {
        // In Rust args[0] is the name of the program
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    run(&buf);
    Ok(())
}

fn run_prompt() -> Result<(), std::io::Error> {
    println!("Running prompt");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line?)
    }
    Ok(())
}

fn run(source: &[u8]) {
    println!("Bytes read from source file: {:?}", source);
}
