use crate::{
    expr::{self, Expr},
    interpreter,
    stmt::Stmt,
    token_type::{
        Token,
        TokenType::{self, *},
    },
};

/*
越往下优先级越高
# 语句
program        → declaration* EOF ;
declaration    → varDecl
               | statement ;
statement      → exprStmt
               | printStmt ;
varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

# 表达式
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")"
               | IDENTIFIER ;
*/

/*

*/

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    stmts.push(stmt);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        if errors.len() == 0 {
            Ok(stmts)
        } else {
            Err(errors.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(Let) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(Identifier, "Expected variable name")?;
        let initializer;
        if self.match_token(Eq) {
            initializer = self.expression()?;
        } else {
            return Err("variiable not found =".to_string());
        }
        self.consume(Semicolon, "Expected ';' after variable declaration")?;

        Ok(Stmt::Var {
            name: token,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(Println) {
            self.expression_print()
        } else {
            self.expression_statement()
        }
    }

    fn expression_print(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Print { expression: expr })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn _synchronize(&mut self) {
        self.advance();
        while self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Let => return,
                _ => (),
            }
            self.advance();
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.comparison()?;
        while self.match_tokens(&[BangEq, EqEq]) {
            let operator = self.previous();
            let rhs = self.comparison()?;

            expr = Expr::Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.term()?;
        while self.match_tokens(&[Greater, GreaterEq, Less, LessEq]) {
            let operator = self.previous();
            let rhs = self.term()?;

            expr = Expr::Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.factor()?;
        while self.match_tokens(&[Plus, Minus]) {
            let operator = self.previous();
            let rhs = self.factor()?;

            expr = Expr::Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.unary()?;
        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous();
            let rhs = self.unary()?;

            expr = Expr::Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary()?;
            Ok(Expr::Unary {
                id: self.get_id(),
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();
        let result;
        match token.token_type {
            Identifier => {
                self.advance();
                result = Expr::Var {
                    id: self.get_id(),
                    value: self.previous(),
                };
            }
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Expr::Grouping {
                    id: self.get_id(),
                    expression: Box::from(expr),
                }
            }
            Number | StringLit => {
                self.advance();
                result = Expr::Literal {
                    id: self.get_id(),
                    value: expr::LiteralValue::from_token(token),
                }
            }
            _ => return Err("Expected expression".to_string()),
        }
        Ok(result)
    }

    // basic
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }
        false
    }

    fn _check(&self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }

    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == typ {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(format!("Line {}: {}", token.line_nu, msg))
        }
    }
}