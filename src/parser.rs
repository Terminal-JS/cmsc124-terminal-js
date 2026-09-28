use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
 
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}