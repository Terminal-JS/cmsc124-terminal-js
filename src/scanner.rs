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

    // method to scan a single token from the source 
    // to be refractored
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Grouping symbols
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            '[' => self.add_token(TokenType::LeftBracket, Literal::None),
            ']' => self.add_token(TokenType::RightBracket, Literal::None),

            // Arithmetic operators
            '+' => self.add_token(TokenType::Plus, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),
            '/' => self.add_token(TokenType::Slash, Literal::None),
            '%' => self.add_token(TokenType::Percent, Literal::None),

            // Boolean operators
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Less, Literal::None);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Equal, Literal::None);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Bang, Literal::None);
                }
            }

            // Separators
            '.' => self.add_token(TokenType::Dot, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),

            // Literals
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),

            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Unrecognized character
            _ => {
                // print an error message for unrecognized characters
                eprintln!("Unexpected character: {}", c);
            }

        }

    }

    fn identifier(&mut self) {
        // Continue consuming characters while they are alphanumeric or underscores
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        // Extract the lexeme from the source
        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = match text.as_str() {
            "var" => TokenType::Var,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "print" => TokenType::Print,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "nil" => TokenType::Nil,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type, Literal::None);
    }

    // checker if the next character matches the expected character
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }
    
}