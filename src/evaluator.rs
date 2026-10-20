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

        Expr::Binary { left, operator, right } => {
            // evaluation order is fixed: left, then right
            let left = evaluate(left)?;
            let right = evaluate(right)?;
 
            match operator.token_type() {
                // + adds two numbers or joins two strings, nothing else
                TokenType::Plus => match (&left, &right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{a}{b}"))),
                    _ => Err(RuntimeError::new(
                        operator,
                        "Operands must be two numbers or two strings.",
                    )),
                },
                TokenType::Minus => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Number(a - b))
                }
                TokenType::Star => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Number(a * b))
                }
                TokenType::Slash => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    if b == 0.0 {
                        return Err(RuntimeError::new(operator, "Division by zero."));
                    }
                    Ok(Value::Number(a / b))
                }

                // still undecided if we include %

                // comparison and equality
                TokenType::Greater => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Bool(a > b))
                }
                TokenType::GreaterEqual => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Bool(a >= b))
                }
                TokenType::Less => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Bool(a < b))
                }
                TokenType::LessEqual => {
                    let (a, b) = check_number_operands(operator, &left, &right)?;
                    Ok(Value::Bool(a <= b))
                }
                // equality works on any pair of types; values of different
                // types are never equal (1 == "1" is false, no error)
                TokenType::EqualEqual => Ok(Value::Bool(left == right)),
                TokenType::BangEqual => Ok(Value::Bool(left != right)),
 
                _ => Err(RuntimeError::new(operator, "Unknown binary operator.")),
            }
        }

        // Sukiscript values
        // no products exist yet until we add statements, so any
        // product lookup fails and revenue/profit are still zero
        Expr::Stock(name) | Expr::Price(name) => Err(RuntimeError::new(
            name,
            &format!("Unknown product '{}'.", name.lexeme()),
        )),
        Expr::Revenue(_) | Expr::Profit(_) => Ok(Value::Number(0.0)),
    }
}

// the scanner already computed the value, so a literal is just a lookup
fn literal_value(token: &Token) -> Value {
    match token.token_type() {
        TokenType::True => Value::Bool(true),
        TokenType::False => Value::Bool(false),
        TokenType::Number => match token.literal() {
            Literal::Number(n) => Value::Number(*n),
            _ => Value::Nil,
        },
        TokenType::String => match token.literal() {
            Literal::Str(s) => Value::Str(s.clone()),
            _ => Value::Nil,
        },
        _ => Value::Nil,
    }
}