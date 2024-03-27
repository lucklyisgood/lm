use crate::stmt::Stmt;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression: _ } => todo!(),
                Stmt::Print { expression } => {
                    println!("{}", expression.evaluate()?.to_string());
                }
                Stmt::Var {
                    name: _,
                    initializer: _,
                } => todo!(),
            }
        }
        Ok(())
    }
}
