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

    // comparison -> term
    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;
 
        while self.match_any(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
 
        Ok(expr)
    }

    // term -> factor
    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;
 
        while self.match_any(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
 
        Ok(expr)
    }

    // factor -> unary
    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;
 
        while self.match_any(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
 
        Ok(expr)
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
 
    fn consume(&mut self, token_type: TokenType, message: &str) -> ParseResult<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }
 
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type() == token_type
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
 
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn error(&self, token: &Token, message: &str) -> ParseError {
        // Report to stderr, exit code 65, per the run contract.
        // Swap this for your group's actual error-reporting function
        // once that's wired up.
        let formatted = if token.token_type() == TokenType::Eof {
            format!("[line {}] Error at end: {}", token.line(), message)
        } else {
            format!("[line {}] Error at '{}': {}", token.line(), token.lexeme(), message)
        };
        eprintln!("{}", formatted);
        ParseError { message: formatted }
    }


}