// [FILENAME]: token_types.rs
// [DESC]:     houses an enum that contains our language's token types

use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // grouping symbols
    LeftParen,   RightParen,
    LeftBrace,   RightBrace,
    LeftBracket, RightBracket,

    // arithmetic operators
    Plus, Minus,
    Star,       // multiplication
    Slash,      // forward slash for division
    Percent,    // modulo

    // boolean operators
    Less,  LessEqual,
    Equal, EqualEqual,
    Bang,       // '!'
    BangEqual,  // '!='

    // separators
    Dot, Comma,

    // SukiScript keywords
    Product,
    Price,
    Stock,
    Sell,
    Quantity,
    Restock,
    Expense,
    Cost,
    Calculate,
    Revenue,
    Profit,
    Show,
    Check,
    Else,

    // kept from before — used if conditionals need booleans, or literal nothing
    True, False,
    Nil,    // or null, from Latin "nihil"

    // literals
    Identifier, // product/customer names
    String,
    Number,

    // end of file
    Eof,
}

impl TokenType {
    pub fn from_keyword(text: &str) -> Option<Self> {
        match text {
            "product" => Some(Self::Product),
            "price" => Some(Self::Price),
            "stock" => Some(Self::Stock),
            "sell" => Some(Self::Sell),
            "quantity" => Some(Self::Quantity),
            "restock" => Some(Self::Restock),
            "expense" => Some(Self::Expense),
            "cost" => Some(Self::Cost),
            "calculate" => Some(Self::Calculate),
            "revenue" => Some(Self::Revenue),
            "profit" => Some(Self::Profit),
            "show" => Some(Self::Show),
            "check" => Some(Self::Check),
            "else" => Some(Self::Else),
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            TokenType::LeftParen  => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace  => "LEFT_BRACE",   
            TokenType::RightBrace => "RIGHT_BRACE",   
            TokenType::LeftBracket => "LEFT_BRACKET",
            TokenType::RightBracket => "RIGHT_BRACKET",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Star => "STAR",
            TokenType::Slash => "SLASH",
            TokenType::Percent => "PERCENT",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Dot => "DOT",
            TokenType::Comma => "COMMA",
            TokenType::Product => "PRODUCT",
            TokenType::Price => "PRICE",
            TokenType::Stock => "STOCK",
            TokenType::Sell => "SELL",
            TokenType::Quantity => "QUANTITY",
            TokenType::Restock => "RESTOCK",
            TokenType::Expense => "EXPENSE",
            TokenType::Cost => "COST",
            TokenType::Calculate => "CALCULATE",
            TokenType::Revenue => "REVENUE",
            TokenType::Profit => "PROFIT",
            TokenType::Show => "SHOW",
            TokenType::Check => "CHECK",
            TokenType::Else => "ELSE",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::Nil => "NIL",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Eof => "EOF",
        };
        write!(f, "{name}")
    }
}