// [FILENAME]: value.rs
// [DESC]:     runtime values produced by the evaluator

use std::fmt;

// a tagged union: one type that can hold any value our language has.
// matching on the tag is how we ask a value what it currently is,
// and the compiler warns us if a case is forgotten
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Value {
    // the one place that decides what counts as true.
    // rule: only false and nil are falsey; everything else
    // (including 0 and "") is truthy
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false))
    }
}

//   numbers: no trailing ".0" (5.0 prints as 5)
//   strings: without quotes
//   nil:     "nil"
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                // the size check keeps huge floats from overflowing i64
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}