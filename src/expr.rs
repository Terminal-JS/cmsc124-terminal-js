// [FILENAME]: expr.rs
// [DESC]:     defines an expression

use crate::token::Token;

pub enum Expr {
    Literal(Token),     // single value
    Unary  { operator: Token, right: Box<Expr> },                   // operator applied to an operand
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },  // operator sandwhiched between two operands
    Grouping(Box<Expr>),    // parenthesized expression
}