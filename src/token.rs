// [FILENAME]:  token.rs
// [DESC]:      houses the Token struct

// import from token_type.rs
use crate::token_type::{self, Literal, TokenType};   // access TokenType and Literal enums

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