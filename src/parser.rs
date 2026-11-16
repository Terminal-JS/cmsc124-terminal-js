use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::stmt::Stmt; 

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,

    // expression mode: line of the first token of the current expression.
    fence_line: usize,

    // true after parse_program() is called, false after parse() is called.
    statement_mode: bool,

    // whether the parser is running in REPL mode or not
    repl: bool,

    block_depth: usize, // tracks the depth of nested blocks

    errors: Vec<ParseError>, // collects errors encountered during parsing
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

 //  ParseResult<T> = one node or one error 
 // ParseResult<T, E> = lets parse() / parse_program() return a vector of nodes or a vector of errors
type ParseResult<T, E = ParseError> = Result<T, E>;

impl Parser {
    // creates an instance of a Parser struct (expression mode by default)
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            fence_line: 0,
            statement_mode: false,
            repl: false,
            block_depth: 0,
            errors: Vec::new(),
        }
    }

    // expression mode 
    pub fn parse(&mut self) -> ParseResult<Vec<Expr>, Vec<ParseError>> {
        let mut exprs = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.expression_line() {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    errors.push(err);
                    self.synchronize()
                }
            }
        }
        if errors.is_empty() { Ok(exprs) } else { Err(errors) }
    }

    // one expression that must end where its line ends
    pub fn expression_line(&mut self) -> ParseResult<Expr> {
        self.fence_line = self.peek().line();

        let expr = self.expression()?;

        // anything else on the same line is leftover
        if !self.is_at_end() && self.peek().line() == self.fence_line {
            // checks wether parser reached end of token stream
            return Err(self.error(self.peek(), "Expect end of expression"));
        }
        Ok(expr)
    }

    // after an error, discard the rest of the bad line 
    // no infinite loop: the first token of every expression
    // is on fence_line, so at least one token is always consumed

    fn synchronize(&mut self) {
        while !self.is_at_end() && self.peek().line() == self.fence_line {
            self.advance();
        }
    }

    // Statement mode: parse a whole program
    // program   -> declaration* EOF
    // Returns every statement if the program is syntactically valid, or every
    // error found. Errors are collected, not fatal: after each one the parser
    // synchronizes and keeps going, so one run reports many mistakes.
    pub fn parse_program(&mut self, repl: bool) -> ParseResult<Vec<Stmt>, Vec<ParseError>> {
        self.statement_mode = true;
        self.repl = repl;
        self.block_depth = 0;
        self.errors.clear();

        let mut stmts = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration_or_recover() {
                stmts.push(stmt);
            }
        }

        if self.errors.is_empty() {
            Ok(stmts)
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    // one declaration, or recover from an error and return None
    fn declaration_or_recover(&mut self) -> Option<Stmt> {
        let before = self.current;
        match self.declaration() {
            Ok(stmt) => Some(stmt),
            Err(err) => {
                self.errors.push(err);
                self.synchronize_stmt(before);
                None
            }
        }
    }

    // declaration -> var_declaration | statement
    fn declaration(&mut self) -> ParseResult<Stmt> {
        // if the next token is 'var', parse a variable declaration, 
        if self.match_any(&[TokenType::Var]) {
            self.var_declaration()
        // otherwise parse a statement
        } else {
            self.statement()   
        }
    }


    // +-------------------------------+
    // | Precedence Chaining           |
    // +-------------------------------+

    // expression -> or
    fn expression(&mut self) -> ParseResult<Expr> {
        self.logic_or()
    }

    // or -> and
    fn logic_or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.logic_and()?;

        while self.match_any(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.logic_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    // and -> comparison
    fn logic_and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;
        
        while self.match_any(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
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

        // a token on a leter line is never part of the current expression
        if self.peek().line() != self.fence_line {
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
        // if the offending token is EOF or on a later line, report at 
        // the end of the line the expression started on
        let ran_out = token.token_type() == TokenType::Eof
            || token.line() != self.fence_line;

        let formatted = if ran_out {
            format!("[line {}] Error at end: {}", self.fence_line, message)
        } else {
            format!("[line {}] Error at '{}': {}", token.line(), token.lexeme(), message)
        };
        eprintln!("{}", formatted);
        ParseError { message: formatted }
    }

    // TODO (week 2):
    // three new methods to be implemented:
        // parse_program()
        // statement()
        // product_declaration()

    // parses a whole program,
    // repl = true allows bare expressions as statements

    // parses a whole program, returning a vector of statements or a vector of errors
    // repl = true allows bare expressions as statements
    pub fn parse_program(&mut self, repl: bool) -> ParseResult<Vec<Stmt>, Vec<ParseError>> {

        // iedit: Result<Vec<Stmt> -> ensure dapat nga node ang ginaretrun 

        self.repl = repl;
        let mut stmts = Vec::new();
        let mut errors = Vec::new();

        // parse statements until the end of the token stream
        while !self.is_at_end() {
            let before = self.current;
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    // report the error and skip to the next statement
                    errors.push(err);
                    self.synchronize_stmt(before);  // skip to the next statement
                }
            }
        }
        // return the vector of statements if there were no errors, otherwise return the vector of errors
        if errors.is_empty() { Ok(stmts) } else { Err(errors) }
    }

    // each statement starts on its own line, so its line becomes the fence
    // method that parses a single statement, returning a Stmt or a ParseError
    fn declaration(&mut self) -> ParseResult<Stmt> {
        self.fence_line = self.peek().line();

        // check which kind of statement it is, and call the appropriate method
        // show, product declaration, sell, restock, expense, calculate, check
        match self.peek().token_type() {
            TokenType::Show => self.show_stmt(),
            // each feature adds its own arm here, one line each
            _ if self.repl => {
                // if in REPL mode, bare expressions are allowed as statements
                let expr = self.expression()?;
                self.end_of_statement()?;
                Ok(Stmt::Expression(expr))
            }
            _ => Err(self.error(self.peek(), "Expect statement.")),
        }
    }

    // parses a show statement, returning a Stmt or a ParseError
    fn show_stmt(&mut self) -> ParseResult<Stmt> {
        self.advance(); // consume 'show'
        let value = self.expression()?;
        self.end_of_statement()?;
        Ok(Stmt::Show(value))
    }

    // after a statement, nothing else may follow on its line (except a closing brace)
    fn end_of_statement(&self) -> ParseResult<()> {
        // if the next token is not EOF, and it is on the same line as the fence, and it is not a closing brace, then it is an error
        let t = self.peek();
        if !self.is_at_end() && t.line() == self.fence_line && t.token_type() != TokenType::RightBrace {
            return Err(self.error(t, "Expect end of statement."));
        }
        Ok(())
    }

    // skip to the next statement keyword. always consumes at least one token,
    // otherwise a bad token loops forever and the harness times out
    fn synchronize_stmt(&mut self, before: usize) {
        if self.current == before {
            self.advance();
        }
        while !self.is_at_end() && !self.is_statement_start() {
            self.advance();
        }
    }

    fn is_statement_start(&self) -> bool {
        // a statement starts with one of these keywords, or a bare expression in REPL mode
        matches!(
            self.peek().token_type(),
            TokenType::Product | TokenType::Sell | TokenType::Restock | TokenType::Expense
                | TokenType::Calculate | TokenType::Show | TokenType::Check
        )
    }

    // todo:
    //  raw helpers
    // check_raw() -> bool, 
    // match_raw() -> bool,
    // consume_raw() -> &Token,
}

   