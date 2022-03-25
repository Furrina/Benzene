use std::env::args;
use std::io::{self, BufRead};

mod token;
mod token_type;

mod scanner;
use scanner::*;

mod error;
use error::*;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len(){
        1=>run_prompt(),
        2=>run_file(&args[1]).expect("Cannot open file"),
        _=>{
            println!("Usage: benzene [script]");
        std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buffer = std::fs::read_to_string(path)?;
    match run(buffer) {
        Ok(_) => {}
        Err(e) => {
            e.report("".to_string());
            std::process::exit(65);
        }
    };

    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!(">");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                    std::process::exit(65);
                }
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), BenzeneError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    if let Ok(token) = tokens {
        for t in token {
            println!("{:?}", t);
        }
    }

    Ok(())
}
