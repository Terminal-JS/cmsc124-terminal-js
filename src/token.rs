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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal_str = match &self.literal {
            // drops .0 when the value is an integer
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            },
            Literal::Str(s)    => s.clone(),
            Literal::Nil       => "null".to_string(),
        };

        write!(f, "Token(type={}, lexeme={}, literal={}, line={})",
            self.token_type, self.lexeme, literal_str, self.line)
    }
}