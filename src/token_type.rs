// [FILENAME]: token_types.rs
// [DESC]:     houses an enum that contains our language's token types

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // grouping symbols
    LeftParen,   RightParen,
    LeftBrace,   RightBrace,
    LeftBracket, RightBracket,

    // arithmetic operators
    Plus, Minus,
    Star,       // multiplication
    Slash,     // forward slash for division
    Percent,    // modulo

    // boolean operators
    Less,  LessEqual,
    Equal, EqualEqual,
    Bang,       // '!'
    BangEqual,  // '!='

    // separators
    Dot, Comma,
    
    // keywords
    Var,
    If, Else,
    While,
    Print,
    True, False,
    Nil,    // or null, from Latin "nihil"

    // literals
    Identifier, // variable/function names
    String,
    Number,

    
    // end of file
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),  
    Nil,
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            TokenType::LeftParen  => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace  => "LEFT_PAREN",
            TokenType::RightBrace => "RIGHT_PAREN",
            TokenType::LeftBracket => "LEFT_BRACKET",
            TokenType::RightBracket => "RIGHT_BRACKET",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Star => "STAR",
            TokenType::Slash => "SLASH",
            TokenType::Percent => "PERCENT",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Dot => "DOT",
            TokenType::Comma => "COMMA",
            TokenType::Var => "VAR",
            TokenType::If => "IF",
            TokenType::Else => "ELSE",
            TokenType::While => "WHILE",
            TokenType::Print => "PRINT",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::Nil => "NIL",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Eof => "EOF",
        };
        write!(f, "{name}");
    }
}