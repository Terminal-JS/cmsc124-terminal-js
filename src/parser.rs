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

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    // creates an instance of a Parser struct
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // entry point: parse a single expression
    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.expression()
    }

    // expression -> equality
    fn expression(&mut self) -> ParseResult<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        
    }

}