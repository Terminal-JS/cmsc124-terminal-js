// [FILENAME]: token_types.rs
// [DESC]:     houses an enum that contains our language's token types

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // grouping symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // arithmetic operators
    Plus,
    Minus,
    Star,       // multiplication
    Slash,     // forward slash for division
    Percent,    // modulo

    // boolean operators
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Bang,       // '!'
    BangEqual,  // '!='

    // separators
    Dot,
    Comma,
    
    // keywords
    Var,
    If,
    Else,
    While,
    Print,
    True,
    False,
    
    Nil,    // or null, from Latin "nihil"

    // literals
    Identifier, // identifier or variables 
    String,
    Number,

    
    // end of file
    Eof,
}

impl TokenType {
    pub fn from_keyword(text: &str) -> Option<Self> {
        match text {
            "var" => Some(Self::Var),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "while" => Some(Self::While),
            "print" => Some(Self::Print),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "nil" => Some(Self::Nil),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),  
    Nil,
}