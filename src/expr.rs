use crate::token_type::{self, Token, TokenType};

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
}

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
}
