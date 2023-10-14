use std::env::args;

pub fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Too many arguments! Usage: lox-ast [script]");
    }
    else if args.len() == 1 {
        // In Rust args[0] is the name of the program
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    println!("Running file: {}", path);
}

fn run_prompt() {
    println!("Running prompt");
}
