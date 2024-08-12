// src/main.rs

mod lexer;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_petra_file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let contents = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    let mut lexer = lexer::Lexer::new(&contents);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}