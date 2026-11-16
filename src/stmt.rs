// [FILENAME]: stmt.rs

use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    // statements and outputs
    Show(Expr),         // evaluates and prints
    Expression(Expr),   // bare expression
    Block(Vec<Stmt>),   // { ... } runs children in a new scope

    // declarations
    Var { name: Token, initializer: Option<Expr> },     // var x; / var x = e;
    Product { name: Token, price: Expr, stock: Expr },
}