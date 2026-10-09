//[FILENAME]: main.rs
//[DESC]:     entry point

// imports
use std::env;   // environment module
use std::fs;    // filesystem module
use std::io::{self, Write};
use scanner::Scanner;

pub mod expr;
pub mod parser;
pub mod print_ast;
pub mod token_type;
pub mod token;
pub mod scanner;


fn main() {
    let args: Vec<String> = env::args().collect();
    const TOKENIZE: &str = "--tokenize";
    const PARSE: &str = "--parse";

    match args.len() {
        1 => run_prompt(),
        2 if args[1] == PARSE => run_parse_prompt(),
        2 => run_program(&args[1]),
        3 if args[1] == TOKENIZE => run_file(&args[2]),
        3 if args[1] == PARSE => run_parse_file(&args[2]),
        _ => {
            eprintln!("Usage: run [<path> | --tokenize <path> | --parse [<path>]]");
            std::process::exit(65);
        }
    }
}


fn run_program(path: &str) {
    fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("lab0: cannot read '{path}': {e}");
            std::process::exit(65);
        });

    println!("Hello, world!");
}


fn run_file(path: &str) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("lab1: cannot read '{path}': {e}");
            std::process::exit(65);
        });
    
    let had_error = run(source);

    if had_error {
        std::process::exit(65);
    }
}


fn run_parse_file(path: &str) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("parser: cannot read '{path}': {e}");
            std::process::exit(65);
        });

    if run_parse_at_line(source, 1) {
        std::process::exit(65);
    }
}


fn run_parse_prompt() {
    let mut line_number = 1;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }

        if !line.trim().is_empty() {
            run_parse_at_line(line, line_number);
        }
        line_number += 1;
    }
}


fn run_parse_at_line(source: String, line: usize) -> bool {
    let mut scanner = Scanner::new_at_line(source, line);
    let tokens = scanner.scan_tokens().clone();
    if scanner.had_error() {
        return true;
    }

    let mut parser = parser::Parser::new(tokens);
    let mut has_error = false;

    while !parser.is_at_end() {
        match parser.parse() {
            Ok(expression) => {
                println!("{}", print_ast::print(&expression));
            }
            Err(_) => {
                has_error = true;
                break;
            }
        }
    }
    has_error
}


fn run_prompt() {
    let mut line_number = 1;

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

        run_at_line(line, line_number);
        line_number += 1;
    }
}


fn run(source: String) -> bool {
    run_at_line(source, 1)
}

fn run_at_line(source: String, line: usize) -> bool {
    let mut scanner = Scanner::new_at_line(source, line); // passes source to scanner constructor
    let tokens = scanner.scan_tokens().clone();
    let had_error = scanner.had_error();

    if !had_error {
        for token in &tokens {
            println!("{}", token);          // ② only print if there was NO error
        }
    }

    had_error
}