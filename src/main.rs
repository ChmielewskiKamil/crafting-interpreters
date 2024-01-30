use std::{
    env::args,
    fs::File,
    io::{BufReader, Read},
};

pub fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Too many arguments! Usage: lox-ast [script]");
    }
    // If user passed in a path to a file
    else if args.len() == 2 {
        // In Rust args[0] is the name of the program
        run_file(&args[1]).expect("Error running file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    run(&buf);
    Ok(())
}

fn run_prompt() {
    println!("Running prompt");
}

fn run(source: &[u8]) {
    println!("Running source");
}
