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

    // equality -> comparison
    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;
 
        while self.match_any(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
 
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {

    }

    // Helper Functions
    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
 
    fn is_at_end(&self) -> bool {
        self.peek().token_type() == TokenType::Eof
    }
 
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    


}