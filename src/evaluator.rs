// [FILENAME]: evaluator.rs
// [DESC]:     walks an expression tree and computes its value
 
use std::fmt;
 
use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::{Literal, TokenType};
use crate::value::Value;
 
// a runtime error remembers the line of the token that caused it,
// which is why the parser stores whole tokens in the tree
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}
 
impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
        RuntimeError { message: message.to_string(), line: token.line() }
    }
}