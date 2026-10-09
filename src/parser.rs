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

                        //  <T: expression, ParseError: message>
type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    // creates an instance of a Parser struct
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // entry point: parse a single expression
    pub fn parse(&mut self) -> ParseResult<Expr> {
        let expr = self.expression()?;
        if !self.is_at_end() {
            // checks wether parser reached end of token stream
            return Err(self.error(self.peek(), "Expect end of expression"));
        }
        Ok(expr)
    }


    // expression -> equality
    fn expression(&mut self) -> ParseResult<Expr> {
        self.equality()
    }

    // equality -> comparison
    fn equality(&mut self) -> ParseResult<Expr> {
        // left comparison
        let mut expr = self.comparison()?;
        
        // checks if it match ( "!=" | "==" )
        while self.match_any(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            
            // left comparison 
            let right = self.comparison()?;

            // creates BT comparison ( ( "!=" | "==" ) comparison )*
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    // comparison -> term
    fn comparison(&mut self) -> ParseResult<Expr> {
        // left term
        let mut expr = self.term()?;
        
        // checks if match any ( ">" | ">=" | "<" | "<=" )
        while self.match_any(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            // right term 
            let right = self.term()?;

            // term ( ( ">" | ">=" | "<" | "<=" ) term )*
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    // term -> factor
    fn term(&mut self) -> ParseResult<Expr> {
        // left factor
        let mut expr = self.factor()?;
        
        // checks if matches any ( "-" | "+" )
        while self.match_any(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            // right factor
            let right = self.factor()?;
            // factor ( ( "-" | "+" ) factor )*
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    // factor -> unary
    fn factor(&mut self) -> ParseResult<Expr> {
        // left unary
        let mut expr = self.unary()?;
 
        while self.match_any(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            // right unary
            let right = self.unary()?;
            // unary ( ( "/" | "*" ) unary )*
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
 
        Ok(expr)
    }

    // unary -> primary
    fn unary(&mut self) -> ParseResult<Expr> {
        // checks if matches any ( "!" | "-" )
        if self.match_any(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            // ( "!" | "-" ) unary
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        
        // ( "!" | "-" ) primary
        self.primary()
    }

// primary -> NUMBER | STRING | "true" | "false" | "nil"
//      | "stock" IDENTIFIER | "price" IDENTIFIER
//      | "revenue" | "profit"
//      | "(" expression ")"
    fn primary(&mut self) -> ParseResult<Expr> {

        // NUMBER | STRING | "true" | "false" | "nil" 
        if self.match_any(&[
            TokenType::Number,
            TokenType::String,
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
        ]) {
            // Expr::Literal wraps the whole token, so we just clone
            // whatever we matched straight into the node.
            return Ok(Expr::Literal(self.previous().clone()));
        }
        
        // "stock" IDENTIFIER
        if self.match_any(&[TokenType::Stock]) {
            let name = self
                .consume(TokenType::Identifier, "Expect product name after 'stock'.")?
                .clone();
            return Ok(Expr::Stock(name));
        }

        // "price" IDENTIFIER
        if self.match_any(&[TokenType::Price]) {
            let name = self
                .consume(TokenType::Identifier, "Expect product name after 'price'.")?
                .clone();
            return Ok(Expr::Price(name));
        } 

        // "revenue" | "profit"
        if self.match_any(&[TokenType::Revenue]) {
            return Ok(Expr::Revenue(self.previous().clone()));
        }

        if self.match_any(&[TokenType::Profit]) {
            return Ok(Expr::Profit(self.previous().clone()));
        }

        // or another expression
        if self.match_any(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
 
        Err(self.error(self.peek(), "Expect expression."))
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
 
   pub fn is_at_end(&self) -> bool {
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