use std::collections::HashMap;

use crate::token_type::{LiteralValue, Token, TokenType};

use TokenType::*;

fn get_keywords() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("println", Println),
        ("let", Let)
    ])
}

pub struct Scanner {
    start: usize,
    cur: usize,
    line: usize,

    source: String,
    tokens: Vec<Token>,
    
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            start: 0,
            cur: 0,
            line: 1,
            tokens: vec![],
            keywords: get_keywords(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while !self.is_at_end() {
            self.start = self.cur;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        if errors.len() > 0 {
            let mut err_str = "".to_string();
            for err in errors {
                err_str.push_str(&err);
                err_str.push_str("\n");
            }
            return Err(err_str);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.cur >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let ch = self.next_char();

        match ch {
            '=' => {
                let token = if self.char_match('=') {
                    EqEq
                } else {
                    Eq
                };
                self.add_token(token);
            },
            ';' => self.add_token(Semicolon),
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,

            '"' => self.string()?,
            c => {
                if c.is_ascii_digit() {
                    self.number()?;
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    return Err(format!("unrecognize char at line {}: {}", self.line, c));
                }
            }
        }
        Ok(())
    }

    fn next_char(&mut self) -> char {
        let c = self.source.chars().nth(self.cur).unwrap();
        self.cur += 1;
        c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = self.source[self.start..self.cur].to_string();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_nu: self.line,
        });
    }

    fn peek_next(&self) -> char {
        if self.cur + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.cur + 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.cur).unwrap()
    }

    fn char_match(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        if self.peek() != ch {
            return false;
        } else {
            self.next_char();
            return true;
        }
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next_char();
        }

        if self.is_at_end() {
            return Err("unterminated string".to_string());
        }

        self.next_char();

        let value = &self.source[self.start + 1 .. self.cur - 1];
        
        self.add_token_lit(StringLit, Some(LiteralValue::StringValue(value.to_string())));

        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while self.peek().is_ascii_digit() {
            self.next_char();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.next_char();
            while self.peek().is_ascii_digit() {
                self.next_char();
            }
        }
        let str_val = &self.source[self.start .. self.cur];
        let val = str_val.parse::<f64>();
        match val {
            Ok(val) => self.add_token_lit(Number, Some(LiteralValue::NumberValue(val))),
            Err(_) => return Err(format!("could not parse number: {}", str_val)),
        }
        Ok(())
    }
    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.next_char();
        }

        let val = &self.source[self.start..self.cur];
        
        if let Some(&ttype) = self.keywords.get(val) {
            self.add_token(ttype);
        } else {
            self.add_token(Identifier);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }