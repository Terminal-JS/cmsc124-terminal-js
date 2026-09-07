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
    FSlash,     // forward slash for division
    Percent,    // modulo

    // boolean operators
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
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



    // literals
    Identifier, // identifier or variables 
    String,
    Number,

    Nil,    // or null, from Latin "nihil"

    // end of file
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),  
    None,   
}