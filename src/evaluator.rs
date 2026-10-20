// [FILENAME]: evaluator.rs
// [DESC]:     walks an expression tree and computes its value
 
use std::fmt;
 
use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::{Literal, TokenType};
use crate::value::Value;
 
// a runtime error remembers the line of the token that caused it,
// which is why the parser stores whole tokens in the tree
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}
 
impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
        RuntimeError { message: message.to_string(), line: token.line() }
    }
}

// display method for runtime errors
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}] Runtime error: {}", self.line, self.message)
    }
}

pub type EvalResult = Result<Value, RuntimeError>;

// evaluates one expression tree and returns its value,
// or a RuntimeError if the tree can't be evaluated
pub fn evaluate(expr: &Expr) -> EvalResult {
    // ask which kind of node we were given, and handle that kind
    match expr {
        // literals, grouping, unary

        // literal_value turns the token into a Value
        Expr::Literal(token) => Ok(literal_value(token)),

        // parentheses only mattered when the parser built the tree.
        // at runtime, just evaluate what is inside them
        Expr::Grouping(inner) => evaluate(inner),

        // unary
        Expr::Unary { operator, right } => {
            // evaluate the operand first, because the operator needs a
            // finished value to work on. if this fails, `?` returns the
            // error from evaluate() immediately.
            // the new `right` is a Value and hides the old `right`,
            // which was a subtree
            let right = evaluate(right)?;
            
            // check which operator it is
            match operator.token_type() {
                // arithmetic negation: only numbers can be negated.
                // check_number_operand returns the f64, or a runtime
                // error carrying the operator's line (so -"hi" fails here)
                TokenType::Minus => {
                    let n = check_number_operand(operator, &right)?;
                    Ok(Value::Number(-n))
                }
                // logical not: (only false and nil are falsey, so !0 is false)
                TokenType::Bang => Ok(Value::Bool(!right.is_truthy())),

                // no other unary operator exists. return an error
                _ => Err(RuntimeError::new(operator, "Unknown unary operator.")),
            }
        }
    }
}