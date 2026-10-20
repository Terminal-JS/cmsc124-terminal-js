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
pub mod value;
pub mod evaluator;

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

    match parse_source(source, 1) {
        Some(trees) => {
            for tree in trees {
                println!("{tree}");
            }
        }
        None => std::process::exit(65),
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

        if let Some(trees) = parse_source(line.trim_end().to_string(), line_number) {
            for tree in trees {
                println!("{tree}");
            }
        }
        
        line_number += 1;
    }
}

// scans the whole source once, then parses one expression per line.
// Some(trees) if every expression passed, None if anything was rejected
fn parse_source(source: String, first_line: usize) -> Option<Vec<String>> {
    let mut scanner = Scanner::new_at_line(source, first_line);
    let tokens = scanner.scan_tokens().clone();
    let lexical_error = scanner.had_error();

    // parse even after a lexical error so syntax errors on
    // other lines still get reported
    let mut parser = parser::Parser::new(tokens);
    let result = parser.parse();

    if lexical_error {
        return None;
    }

    match result {
        Ok(exprs) => Some(exprs.iter().map(print_ast::print).collect()),
        Err(_) => None,
    }
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