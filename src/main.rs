//[FILENAME]: main.rs
//[DESC]:     entry point

// imports
use std::env;   // environment module
use std::fs;    // filesystem module
use std::io::{self, Write};
use scanner::Scanner;
use expr::Expr;
use crate::evaluator::RuntimeError;
use crate::value::Value;

const EXIT_USAGE:   i32 = 64;   // bad CL args or incorrect usage
const EXIT_DATA:    i32 = 65;   // file I/O failure, scanner/parse error
const EXIT_RUNTIME: i32 = 70;   // started, then failed partway
const INPUT_INDICATOR: &str = "> ";

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
    const EVAL: &str = "--eval";
    
    match args.len() {
        // no arguments: run the REPL
        1 => run_eval_prompt(), 
        2 if args[1] == EVAL => run_eval_prompt(),  
        2 if args[1] == PARSE => run_parse_prompt(),
        2 if args[1] == TOKENIZE => run_prompt(),
        2 => run_program(&args[1]),
        3 if args[1] == TOKENIZE => run_file(&args[2]),
        3 if args[1] == PARSE => run_parse_file(&args[2]),
        3 if args[1] == EVAL => run_eval_file(&args[2]),
        _ => {
            eprintln!("Usage: run [<path> | --tokenize [<path>] | --parse [<path>] | --eval [<path>]]");
            std::process::exit(EXIT_USAGE);
        }
    }
}


// HELPER:  method for verifying if path exists/valid
// returns: exit code 65 on bad file path
fn read_source(label: &str, path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("{label}: cannot read '{path}': '{e}'");
        std::process::exit(EXIT_DATA);
    })
}


// runs a program from a file, or prints an error and exits if the file can't be read
fn run_program(path: &str) {
    read_source("lab0", path);
    println!("Hello, world!");
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("lab1: cannot read '{path}': {e}");
            std::process::exit(EXIT_DATA);
        });
    
    let had_error = run(source);

    if had_error {
        std::process::exit(EXIT_DATA);
    }
}

fn run_parse_file(path: &str) {
    let source = read_source("parser", path);
    match parse_source(source, 1) {
        Some(trees) => {
            for tree in trees {
                println!("{tree}");
            }
        }
        None => std::process::exit(EXIT_DATA),
    }
}

// REPL for --parse: prints the tree for each line types
// bad line prints its error and returns
fn run_parse_prompt() {
    run_repl(|line, n| {
        if let Some(trees) = parse_source(line, n) {
            for tree in trees {
                println!("{tree}");
            }
        }
    });
}

// scans the whole source once, then parses one expression per line.
// Some(trees) if every expression passed, None if anything was rejected
// scans and parses; Some(exprs) if no lexical or syntax error
fn parse_exprs(source: String, first_line: usize) -> Option<Vec<Expr>> {
    let mut scanner = Scanner::new_at_line(source, first_line);
    let tokens = scanner.scan_tokens().clone();
    let lexical_error = scanner.had_error();

    let mut parser = parser::Parser::new(tokens);
    let result = parser.parse();

    if lexical_error {
        return None;
    }
    result.ok()
}

fn parse_source(source: String, first_line: usize) -> Option<Vec<String>> {
    parse_exprs(source, first_line)
        .map(|exprs| exprs.iter().map(print_ast::print).collect())
}




fn run_eval_file(path: &str) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("eval: cannot read '{path}': '{e}'");
            std::process::exit(EXIT_DATA);
        });

    // static error: nothing runs, exit EXIT_DATA
    let exprs = match parse_exprs(source, 1) {
        Some(exprs) => exprs,
        None => std::process::exit(EXIT_DATA),
    };


    // values print as we go; the first runtime error stops the run
    if let Err(err) = evaluator::evaluate_all(&exprs, |value| println!("{value}")) {
        eprintln!("{err}");
        std::process::exit(EXIT_RUNTIME);
    }

}




// REPL for --eval: prints the value of each line typed
// runtime error is printed and session continues
// file mode exits 70
fn run_eval_prompt() {
    run_repl(|line, n| {
        if let Some(exprs) = parse_exprs(line, n) {
            if let Err(err) = evaluator::evaluate_all(&exprs, |value| println!("{value}")) {
                eprintln!("{err}"); // report it and return to the prompt
            }
        };
    })
}

// REPL for --tokenize: prints the tokens of each line typed
fn run_prompt() {
    run_repl(|line, n| {
        run_at_line(line, n);
    });
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
            println!("{}", token);          // only print if there was NO error
        }
    }

    had_error
}


// prompts, reads, trims, and numbers lines
// all REPL mode calls this and supplies only 'handle'
fn run_repl(mut handle: impl FnMut(String, usize)) {
    let mut line_number = 1;

    loop {
        print!("{}", INPUT_INDICATOR);
        // failed flush delays prompt (so not fatal)
        let _ = io::stdout().flush();


        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("repl: cannot read input {e}");
                break;
            }
        }

        handle(line.trim_end().to_string(), line_number);
        line_number += 1;
    }
}