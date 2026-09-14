// [FILENAME]:  token.rs
// [DESC]:      houses the Token struct

// imports
use std::fmt;   // format module
use crate::token_type::{Literal, TokenType};   // access TokenType and Literal enums

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme:     String,
    literal:    Literal,
    line:       usize,
}

impl Token {
    // constructor that creates and returns a new Token
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Literal,
        line: usize,
    ) -> Self {
        // Create and return a new Token instance
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    // getter for token_type
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    // getter for lexeme
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }   

    // getter for literal
    pub fn literal(&self) -> &Literal {
        &self.literal
    }   

    // getter for line
    pub fn line(&self) -> usize {
        self.line
    }   

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal_str = match &self.literal {
            Literal::Number(n) => format!("{:?}", n),
            Literal::Str(s)    => s.clone(),
            Literal::Nil       => "null".to_string(),
        };

        write!(f, "Token(type={}, lexeme={}, literal={}, line={})",
            self.token_type, self.lexeme, literal_str, self.line)
    }
}