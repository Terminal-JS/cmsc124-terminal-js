// [FILENAME]: print_ast.rs
// [DESC]:     prints the Abstract Syntax Tree as an algebraic-like expression

use crate::expr::Expr;
use crate::token_type::Literal;

pub fn print(expr: &Expr) -> String {
	// parameter takes a reference to an expression
    
	match expr {
		// pattern match to different variants of expression (Expr) enum
        Expr::Literal(token)  => print_literal(token.literal()),
        Expr::Grouping(inner) => format!("(group {})", print(inner)),
        
		Expr::Unary { operator, right } => {
			format!("({} {})", operator.lexeme(), print(right))
		}
		Expr::Binary { left, operator, right } => {
			format!("({} {} {})", operator.lexeme(), print(left), print(right))
		}
    }
}

// helper function to convert raw primitive literal types to String
fn print_literal(literal: &Literal) -> String {
	match literal {
		Literal::Number(n) => format!("{n:?}"),	 // converts the number to a string
		Literal::Str(s)	   => s.clone(),		 // returns a cloned, owned String instance
		Literal::Nil  	   => "nil".to_string(), // return owned string "nil"
	}
}