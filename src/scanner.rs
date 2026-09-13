// [FILENAME]: scanner.rs
// [DESC]:     scans source text into a stream of tokens

use crate::token::Token;
use crate::token_type::{Literal, TokenType};


pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
}

impl Scanner {
    // constructor that creates and returns a new Scanner
    pub fn new(source: String) -> Self {
        Scanner {
            // String -> Vec<char> for char-by-char processing
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn had_error(&self) -> bool {
        // lets main() check after scanning
        self.had_error
    }

    // helper method to check if we've reached the end of the source
    fn is_at_end(&self) -> bool {
        // > prevents out-of-bounds access
        self.current >= self.source.len()
    }

    // method to scan the source text and produce a vector of tokens
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // we are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        // add an EOF token at the end of the token stream
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::Nil,
            self.line,
        ));

        &self.tokens
    }

    // method to scan a single token from the source 
    // to be refractored
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Grouping symbols
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Nil),
            '}' => self.add_token(TokenType::RightBrace, Literal::Nil),
            '[' => self.add_token(TokenType::LeftBracket, Literal::Nil),
            ']' => self.add_token(TokenType::RightBracket, Literal::Nil),

            // Arithmetic operators
            '+' => self.add_token(TokenType::Plus, Literal::Nil),
            '-' => self.add_token(TokenType::Minus, Literal::Nil),
            '*' => self.add_token(TokenType::Star, Literal::Nil),
            '/' => self.scan_slash(),
            '%' => self.add_token(TokenType::Percent, Literal::Nil),

            // Boolean operators
            '<' | '=' | '!' => self.scan_operator(c),

            // Separators
            '.' => self.add_token(TokenType::Dot, Literal::Nil),
            ',' => self.add_token(TokenType::Comma, Literal::Nil),

            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => self.identifier(),

            // Ignore whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Unrecognized character
            _ => {
                eprintln!("[line {}] Error: Unexpected character.", self.line);
                self.had_error = true;
            }
        }
    }

    // helper method to scan operators that may be 
    // single or double character tokens
    fn scan_operator(&mut self, first: char) {
        let token_type = match (first, self.match_char('=')) {
            ('<', true) => TokenType::LessEqual,
            ('<', false) => TokenType::Less,
            ('=', true) => TokenType::EqualEqual,
            ('=', false) => TokenType::Equal,
            ('!', true) => TokenType::BangEqual,
            ('!', false) => TokenType::Bang,
            _ => unreachable!(),
        };

        self.add_token(token_type, Literal::Nil);
    }

    // handles '/' after it's already been consumed by advance() in scan_token.
    // disambiguates ordinary division from a "//" line comment by looking
    // one character ahead.
    fn scan_slash(&mut self) {
        if self.match_char('/') {
            // no token is added here
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
        } else {
            self.add_token(TokenType::Slash, Literal::Nil);
        }
    }

    fn identifier(&mut self) {
        // scans an identifier or keyword from the source text
        while self.peek().is_alphanumeric()
                    || self.peek() == '_' {
            self.advance();
        }

        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        let token_type = TokenType::from_keyword(&text)
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type, Literal::Nil);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // consume .

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let text: String = self.source[self.start..self.current]
            .iter()
            .collect();

        let value: f64 = text
            .parse()
            .expect(&format!("invalid number literal: {:?}", text));

        self.add_token(TokenType::Number, Literal::Number(value));
    }

    // checker if the next character matches the expected character
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }


    fn advance(&mut self) -> char {
        // advances the cursor to the next character
        let c = self.source[self.current];
        self.current += 1;
        c   // return c
    }

    fn peek(&self) -> char{
        // reads and returns the character ahead of current
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        // reads the character one past current
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        // bundles scanned lexeme into a structured token object
        // and appends to list of output tokens
        let text: String = self.source[self.start..self.current]
            .iter()
            .collect();
        
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::token_type::TokenType;

    #[test]
    fn skips_line_comments_and_counts_their_newline() {
        let mut scanner = Scanner::new("var first // ignored\nvar second".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type(), TokenType::Var);
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[1].token_type(), TokenType::Identifier);
        assert_eq!(tokens[1].lexeme(), "first");
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[2].token_type(), TokenType::Var);
        assert_eq!(tokens[2].line(), 2);
        assert_eq!(tokens[3].lexeme(), "second");
        assert_eq!(tokens[3].line(), 2);
    }

    #[test]
    fn keeps_a_single_slash_as_division() {
        let mut scanner = Scanner::new("a / b".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[1].token_type(), TokenType::Slash);
        assert_eq!(tokens[1].lexeme(), "/");
    }
}