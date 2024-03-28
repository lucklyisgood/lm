use crate::token_type::{self, Token, TokenType};

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
}

use LiteralValue::*;

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn unwrap_as_f64(literal: Option<token_type::LiteralValue>) -> f64 {
    match literal {
        Some(token_type::LiteralValue::NumberValue(x)) => x as f64,
        _ => panic!("Could not unwrap as f64"),
    }
}

fn unwrap_as_string(literal: Option<token_type::LiteralValue>) -> String {
    match literal {
        Some(token_type::LiteralValue::StringValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => format!("\"{}\"", x),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal)),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        id: usize,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
    Literal {
        id: usize,
        value: LiteralValue,
    },
    Var {
        id: usize,
        value: Token,
    },
}

impl Expr {
    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&left, operator.token_type, &right) {
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (x, ttype, y) => Err(format!(
                        "{} is not implemented for operands {:?} and {:?}",
                        ttype, x, y
                    )),
                }
            }
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = right.evaluate()?;
                match (&right, operator.token_type) {
                    (LiteralValue::Number(v), TokenType::Minus) => Ok(LiteralValue::Number(-v)),
                    // (any, Token::Bang) => Ok(any.)
                    (_, ttype) => Err(format!("{} is not a valid unary operator", ttype)),
                }
            }
            Expr::Grouping { id: _, expression } => expression.evaluate(),
            Expr::Literal { id: _, value } => Ok((*value).clone()),
            Expr::Var { id: _, value } => todo!(),
        }
    }
}
