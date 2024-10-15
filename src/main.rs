use std::path;

use clap::Parser;
use petram::lexer;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    program: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let program_path = path::Path::new(args.program.as_str())
        .canonicalize()
        .expect("Could not canonicalize path");

    println!("Program path: {}", program_path.display());

    if let Ok(source) = std::fs::read_to_string(program_path) {
        let mut lexer = lexer::Lexer::new(&source);
        let mut tokens: Vec<lexer::Token> = vec![];

        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        tokens.iter().for_each(|token| println!("{}", token));
        return Ok(());
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not read file",
        ));
    }
}
