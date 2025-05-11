use crate::token::{Token, TokenType, Literal};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Variable(Token),
    Array(Token, Vec<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Loop(Option<Expr>, Vec<Stmt>),
    Break,
    Function(Token, Vec<Token>, Expr),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Ts) {
            self.var_declaration()
        } else if self.match_token(TokenType::Hawk) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(
            TokenType::Identifier,
            "Expected variable name.".to_string(),
        )?;

        let mut initializer = None;

        if self.match_token(TokenType::Pmo) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.".to_string(),
        )?;

        Ok(Stmt::Var(name, initializer))
    }

    fn function_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(
            TokenType::Identifier,
            "Expected function name.".to_string(),
        )?;

        self.consume(
            TokenType::LeftParen,
            "Expected '(' after function name.".to_string(),
        )?;

        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                parameters.push(self.consume(
                    TokenType::Identifier,
                    "Expected parameter name.".to_string(),
                )?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            "Expected ')' after parameters.".to_string(),
        )?;

        self.consume(
            TokenType::Tuah,
            "Expected 'tuah' after function parameters.".to_string(),
        )?;

        let body = self.expression()?;

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after function body.".to_string(),
        )?;

        Ok(Stmt::Function(name, parameters, body))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Yap) {
            self.print_statement()
        } else if self.match_token(TokenType::Yo) {
            self.if_statement()
        } else if self.match_token(TokenType::Goon) {
            self.loop_statement()
        } else if self.match_token(TokenType::Sybau) {
            self.break_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after value.".to_string(),
        )?;

        Ok(Stmt::Print(value))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.match_token(TokenType::Gurt) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn loop_statement(&mut self) -> Result<Stmt, String> {
        let mut condition = None;

        // Check if it's a goon(n) style loop
        if self.match_token(TokenType::LeftParen) {
            condition = Some(self.expression()?);
            self.consume(
                TokenType::RightParen,
                "Expected ')' after loop condition.".to_string(),
            )?;
        }

        let mut body = Vec::new();

        while !self.check(TokenType::Edge) && !self.is_at_end() {
            body.push(self.declaration()?);
        }

        self.consume(
            TokenType::Edge,
            "Expected 'edge' after loop body.".to_string(),
        )?;

        Ok(Stmt::Loop(condition, body))
    }

    fn break_statement(&mut self) -> Result<Stmt, String> {
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after 'sybau'.".to_string(),
        )?;

        Ok(Stmt::Break)
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression.".to_string(),
        )?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?;

        if self.match_token(TokenType::Pmo) {
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Binary(Box::new(Expr::Variable(name)),
                                      self.previous(),
                                      Box::new(value)));
            }

            return Err("Invalid assignment target.".to_string());
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash, TokenType::Modulo]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(0.0))),
                operator,
                Box::new(right),
            ));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expected ')' after arguments.".to_string(),
        )?;

        Ok(Expr::Call(Box::new(callee), paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(TokenType::String) || self.match_token(TokenType::Number) {
            if let Some(literal) = &self.previous().literal {
                return Ok(Expr::Literal(literal.clone()));
            }
        } else if self.match_token(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous()));
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expected ')' after expression.".to_string(),
            )?;
            return Ok(Expr::Grouping(Box::new(expr)));
        } else if self.match_token(TokenType::Gyat) {
            return self.array();
        } else if self.match_token(TokenType::Yeet) {
            return Ok(Expr::Literal(Literal::String("__YEET__".to_string())));  // Special marker for input
        }

        Err(format!("Expected expression, got {:?}", self.peek()))
    }

    fn array(&mut self) -> Result<Expr, String> {
        // Consume the array name
        let name = self.consume(
            TokenType::Identifier,
            "Expected array name after 'gyat'.".to_string()
        )?;

        // Consume the opening brace
        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after array name.".to_string(),
        )?;

        let mut elements = Vec::new();

        if !self.check(TokenType::RightBrace) {
            loop {
                elements.push(self.expression()?);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightBrace,
            "Expected '}' after array elements.".to_string(),
        )?;

        Ok(Expr::Array(name, elements))
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} Got {:?}", message, self.peek()))
        }
    }
}
