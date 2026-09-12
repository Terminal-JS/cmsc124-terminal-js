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
    had_error: bool,
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
            had_error: false,
        }
    }

    pub fn had_error(&self) -> bool {
        // lets main() check after scanning
        self.had_error
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
            Literal::Nil,
            self.line,
        ));

        &self.tokens
    }

    // method to scan a single token from the source 
    // to be refractored
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Grouping symbols
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Nil),
            '}' => self.add_token(TokenType::RightBrace, Literal::Nil),
            '[' => self.add_token(TokenType::LeftBracket, Literal::Nil),
            ']' => self.add_token(TokenType::RightBracket, Literal::Nil),

            // Arithmetic operators
            '+' => self.add_token(TokenType::Plus, Literal::Nil),
            '-' => self.add_token(TokenType::Minus, Literal::Nil),
            '*' => self.add_token(TokenType::Star, Literal::Nil),
            '/' => self.add_token(TokenType::Slash, Literal::Nil),
            '%' => self.add_token(TokenType::Percent, Literal::Nil),

            // Boolean operators 
            '<' => self.add_token(TokenType::Less, Literal::Nil),
            '=' => self.add_token(TokenType::Equal, Literal::Nil),
            '!' => self.add_token(TokenType::Bang, Literal::Nil),

            // Separators
            '.' => self.add_token(TokenType::Dot, Literal::Nil),
            ',' => self.add_token(TokenType::Comma, Literal::Nil),

            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Unrecognized character
            _ => {
                eprintln!("[line {}] Error: Unexpected character.", self.line);
                self.had_error = true;
            }
        }
    }

    // checker if the next character matches the expected character
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }


    fn advance(&mut self) -> char {
        // advances the cursor to the next character
        let c = self.source[self.current];
        self.current += 1;
        c   // return c
    }

    fn peek(&self) -> char{
        // reads and returns the character ahead of current
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        // bundles scanned lexeme into a structured token object
        // and appends to list of output tokens
        let text: String = self.source[self.start..self.current]
            .iter()
            .collect();
        
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

}