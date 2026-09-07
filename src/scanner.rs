// [FILENAME]: scanner.rs
// [DESC]:     scans source text into a stream of tokens

use crate::token::Token;
use crate::token_type::{Literal, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    // constructor that creates and returns a new Scanner
    pub fn new(source: String) -> Self {
        Scanner {
            // String -> Vec<char> for char-by-char processing
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // helper method to check if we've reached the end of the source
    fn is_at_end(&self) -> bool {
        // > prevents out-of-bounds access
        self.current >= self.source.len()
    }

    // method to scan the source text and produce a vector of tokens
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // we are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        // add an EOF token at the end of the token stream
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::None,
            self.line,
        ));

        &self.tokens;
    }



    
}