// [FILENAME]: stmt.rs

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Show(Expr),         // evaluates and prints
    Expression(Expr),   // bare expression
    Block(Vec<Stmt>)    // basically: {...}, run chilren in order
}