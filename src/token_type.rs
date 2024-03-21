use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // 单一字符
    Semicolon,
    LeftParen,
    RightParen,

    // 一个或多个字符
    Eq,
    EqEq,

    // 字面量(Literals)
    Identifier,
    StringLit,
    Number,

    // 关键字
    Println,
    Let,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    NumberValue(f64),
    StringValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_nu: usize,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}