// [FILENAME]: stmt.rs

use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    // statements and outputs
    Show(Expr),         // evaluates and prints
    Expression(Expr),   // bare expression
    Block(Vec<Stmt>),    // basically: {...}, run chilren in order

    // declarations
    Product {name: Token, price: Expr, Stock: Expr},
}