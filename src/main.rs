use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
    // Single character tokens
    Plus,
    Minus,
    Slash,
    Star,

    // One or two character tokens
    Equal,

    // Literals
    Number,
    String,
    Identifier,

    // Keywords
    Function,
    True,
    False,

    EOF,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Location {
    // Offset from the beginning of the file to the beginning of the lexeme
    offset: u32,
    // Length of the lexeme
    length: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    location: Location,
}

#[allow(dead_code)]
impl Token {
    fn to_string(&self) -> String {
        format!(
            "Token: {:?}, Lexeme: {}, Location: {:?}",
            self.token_type, self.lexeme, self.location
        )
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
