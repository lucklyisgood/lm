use crate::{expr::Expr, token_type::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
}
