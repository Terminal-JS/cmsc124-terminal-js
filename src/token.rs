// [FILENAME]:  token.rs
// [DESC]:      houses the Token struct

// import from token_type.rs
use crate::token_type::{TokenType, Literal};   // access TokenType and Literal enums

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme:     String,
    literal:    Literal,
    line:       usize,
}