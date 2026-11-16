// [FILENAME]: stmt.rs

use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    // statements and outputs
    Show(Expr),       // evaluates and prints
    Expression(Expr), // bare expression
    Block(Vec<Stmt>), // basically: {...}, run chilren in order

    // loops
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },

    Check {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },

    // declarations
    Product {
        name: Token,
        price: Expr,
        Stock: Expr,
    },
}
