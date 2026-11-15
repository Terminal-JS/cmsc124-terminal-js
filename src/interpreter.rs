// [FILENAME]: interpreter.rs

pub struct Interpreter {
    env: Environment,
    revenue: f64,
    costs: f64,
    expenses: f64,
}

impl Interpreter {
    pub fn new() -> Self {
        todo!()
    }
    
    
    pub fn evaluate(&self, expr: &Expr) -> EvalResult {
        todo!()
    }   // your old evaluate, moved

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        todo!()
    }

    fn exec_product(&mut self, name: &Token, price: &Expr, stock: &Expr) -> Result<(), RuntimeError> {
            todo!()
    }
}