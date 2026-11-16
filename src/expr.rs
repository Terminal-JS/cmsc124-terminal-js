// [FILENAME]: expr.rs
// [DESC]:     defines an expression

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Token),     // single value
    Unary  { operator: Token, right: Box<Expr> },                   // operator applied to an operand
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },  // operator sandwiched between two operands
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> }, // and / or (short-circuit)
    Grouping(Box<Expr>),    // parenthesized expression

    // variables 
    Variable(Token),                                // read a name
    Assign { name: Token, value: Box<Expr> },       // name = value

    // SukiScript specific keywords
    Stock(Token),
    Price(Token),
    Revenue(Token),
    Profit(Token),
}