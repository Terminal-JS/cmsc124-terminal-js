// [FILENAME]: expr.rs
// [DESC]:     defines an expression

use crate::token::Token;

pub enum Expr {
    Literal(Token),     // single value
    Unary  { operator: Token, right: Box<Expr> },                   // operator applied to an operand
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },  // operator sandwhiched between two operands
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr>},  // logical operations
    Grouping(Box<Expr>),    // parenthesized expression

    // SukiScript specific keywords
    Stock(Token), 
    Price(Token),
    Revenue(Token),
    Profit(Token),
}