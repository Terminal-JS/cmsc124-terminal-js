//[FILENAME]: main.rs
//[DESC]:     entry point

// imports
use std::env;   // environment module
use std::fs;    // filesystem module
use std::io::{self, Write};
use scanner::Scanner;

pub mod token_type;
pub mod token;
pub mod scanner;


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        3 if args[1] == "--tokenize" => run_file(&args[2]),
        _ => {
            eprintln!("Usage: run [--tokenize <path>]");
            std::process::exit(65);
        }
    }
}


fn run_file(path: &str) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("lab1: cannot read '{path}': {e}");
            std::process::exit(65); // Exit Code 74: I/O file error
        });
    
    let had_error = run(source);

    if had_error {
        std::process::exit(65);
    }
}


fn run_prompt() {
    loop {
        // flushes cursor on the same
        // line as the input indicator
        print!("> ");
        io::stdout()
            .flush()
            .unwrap();

        let mut line = String::new();

        // read lines from terminal
        let bytes_read = io::stdin()
                            .read_line(&mut line)
                            .unwrap();

        if bytes_read == 0 {
            break;  // EOF
        }

        run(line);
    }
}


fn run(source: String) -> bool {
    let mut scanner = Scanner::new(source); // passes source to scanner constructor
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    scanner.had_error()
}