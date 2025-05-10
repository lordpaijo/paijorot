use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::process;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Token {
    Yap,
    Ts,
    Pmo,
    Gyat,
    Hawk,
    Tuah,
    Goon,
    GoonN,
    Edge,
    Yeet,
    Sybau,
    Identifier(String),
    String(String),
    Number(f64),
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    And,
    Or,
    Comment,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Box<Expression>,
    },
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Function { .. } => write!(f, "<function>"),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Literal(Value),
    Identifier(String),
    Binary(Box<Expression>, String, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    Array(Vec<Expression>),
    Input,
}

#[derive(Debug)]
enum Statement {
    Declaration(String, Expression),
    Print(Expression),
    Function(String, Vec<String>, Expression),
    Loop(Option<Expression>, Vec<Statement>),
    Break,
    Expression(Expression),
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        identifier
    }

    fn read_number(&mut self) -> f64 {
        let mut number = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) || ch == '.' {
                number.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        number.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Skip opening quote
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance(); // Skip backslash
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => string.push(escaped),
                    }
                }
            } else {
                string.push(self.advance().unwrap());
            }
        }
        string
    }

    fn read_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.peek() {
            None => None,
            Some(ch) => {
                match ch {
                    '(' => {
                        self.advance();
                        Some(Token::LParen)
                    }
                    ')' => {
                        self.advance();
                        Some(Token::RParen)
                    }
                    '{' => {
                        self.advance();
                        Some(Token::LBrace)
                    }
                    '}' => {
                        self.advance();
                        Some(Token::RBrace)
                    }
                    '[' => {
                        self.advance();
                        Some(Token::LBracket)
                    }
                    ']' => {
                        self.advance();
                        Some(Token::RBracket)
                    }
                    ',' => {
                        self.advance();
                        Some(Token::Comma)
                    }
                    ';' => {
                        self.advance();
                        Some(Token::Semicolon)
                    }
                    '+' => {
                        self.advance();
                        Some(Token::Plus)
                    }
                    '-' => {
                        self.advance();
                        Some(Token::Minus)
                    }
                    '*' => {
                        self.advance();
                        Some(Token::Multiply)
                    }
                    '/' => {
                        self.advance();
                        if self.peek() == Some('/') {
                            self.advance();
                            self.read_comment();
                            Some(Token::Comment)
                        } else {
                            Some(Token::Divide)
                        }
                    }
                    '=' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Some(Token::Equals)
                        } else {
                            Some(Token::Equals) // Still using Equals for now
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Some(Token::NotEquals)
                        } else {
                            // Handle ! operator if needed
                            Some(Token::Identifier("!".to_string()))
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Some(Token::LessThanEq)
                        } else {
                            Some(Token::LessThan)
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Some(Token::GreaterThanEq)
                        } else {
                            Some(Token::GreaterThan)
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.peek() == Some('&') {
                            self.advance();
                            Some(Token::And)
                        } else {
                            Some(Token::Identifier("&".to_string()))
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.peek() == Some('|') {
                            self.advance();
                            Some(Token::Or)
                        } else {
                            Some(Token::Identifier("|".to_string()))
                        }
                    }
                    '"' => Some(Token::String(self.read_string())),
                    _ => {
                        if ch.is_alphabetic() {
                            let identifier = self.read_identifier();
                            match identifier.as_str() {
                                "yap" => Some(Token::Yap),
                                "ts" => Some(Token::Ts),
                                "pmo" => Some(Token::Pmo),
                                "gyat" | "gyatt" => Some(Token::Gyat),
                                "hawk" => Some(Token::Hawk),
                                "tuah" => Some(Token::Tuah),
                                "goon" => Some(Token::Goon),
                                "edge" => Some(Token::Edge),
                                "yeet" => Some(Token::Yeet),
                                "sybau" => Some(Token::Sybau),
                                _ => Some(Token::Identifier(identifier)),
                            }
                        } else if ch.is_digit(10) {
                            Some(Token::Number(self.read_number()))
                        } else {
                            self.advance(); // Skip unrecognized characters
                            self.next_token()
                        }
                    }
                }
            }
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if !matches!(token, Token::Comment) {
                tokens.push(token);
            }
        }
        tokens
    }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(token) == std::mem::discriminant(expected) {
                self.advance();
                return Ok(());
            }
            return Err(format!("Expected {:?}, got {:?}", expected, token));
        }
        Err("Unexpected end of input".to_string())
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Ok(Expression::Literal(Value::Number(n)))
            }
            Some(Token::String(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::Literal(Value::String(s)))
            }
            Some(Token::Identifier(id)) => {
                let id = id.clone();
                self.advance();

                // Check if it's a function call
                if let Some(Token::LParen) = self.peek() {
                    self.advance();
                    let mut args = Vec::new();

                    // Parse arguments
                    if let Some(Token::RParen) = self.peek() {
                        self.advance();
                    } else {
                        loop {
                            args.push(self.parse_expression()?);

                            match self.peek() {
                                Some(Token::Comma) => {
                                    self.advance();
                                }
                                Some(Token::RParen) => {
                                    self.advance();
                                    break;
                                }
                                _ => return Err("Expected ',' or ')' in function call".to_string()),
                            }
                        }
                    }

                    Ok(Expression::FunctionCall(id, args))
                } else {
                    // Check if it's an array access
                    if let Some(Token::LBracket) = self.peek() {
                        self.advance();
                        let index = self.parse_expression()?;

                        if let Some(Token::RBracket) = self.peek() {
                            self.advance();
                            Ok(Expression::Binary(
                                Box::new(Expression::Identifier(id)),
                                "[]".to_string(),
                                Box::new(index)
                            ))
                        } else {
                            Err("Expected ']' in array access".to_string())
                        }
                    } else {
                        Ok(Expression::Identifier(id))
                    }
                }
            }
            Some(Token::Yeet) => {
                self.advance();
                Ok(Expression::Input)
            }
            Some(Token::Gyat) => {
                self.advance();

                // Parse array name (optional)
                let _name = if let Some(Token::Identifier(id)) = self.peek() {
                    let id = id.clone();
                    self.advance();
                    Some(id)
                } else {
                    None
                };

                self.expect(&Token::LBrace)?;

                let mut elements = Vec::new();

                if let Some(Token::RBrace) = self.peek() {
                    self.advance();
                } else {
                    loop {
                        elements.push(self.parse_expression()?);

                        match self.peek() {
                            Some(Token::Comma) => {
                                self.advance();
                            }
                            Some(Token::RBrace) => {
                                self.advance();
                                break;
                            }
                            _ => return Err("Expected ',' or '}' in array".to_string()),
                        }
                    }
                }

                Ok(Expression::Array(elements))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek())),
        }
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_expression()?;

        while let Some(token) = self.peek() {
            let current_precedence = match token {
                Token::Plus | Token::Minus => 1,
                Token::Multiply | Token::Divide => 2,
                Token::Equals | Token::NotEquals | Token::LessThan | Token::GreaterThan | Token::LessThanEq | Token::GreaterThanEq => 0,
                _ => 0,
            };

            if current_precedence <= precedence {
                break;
            }

            let op = match self.advance() {
                Some(Token::Plus) => "+".to_string(),
                Some(Token::Minus) => "-".to_string(),
                Some(Token::Multiply) => "*".to_string(),
                Some(Token::Divide) => "/".to_string(),
                Some(Token::Equals) => "==".to_string(),
                Some(Token::NotEquals) => "!=".to_string(),
                Some(Token::LessThan) => "<".to_string(),
                Some(Token::GreaterThan) => ">".to_string(),
                Some(Token::LessThanEq) => "<=".to_string(),
                Some(Token::GreaterThanEq) => ">=".to_string(),
                _ => return Err("Expected operator".to_string()),
            };

            let right = self.parse_binary_expression(current_precedence)?;
            left = Expression::Binary(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(Token::Yap) => {
                self.advance();
                let expr = self.parse_binary_expression(0)?;
                self.expect(&Token::Semicolon)?;
                Ok(Statement::Print(expr))
            }
            Some(Token::Ts) => {
                self.advance();

                let name = match self.advance() {
                    Some(Token::Identifier(id)) => id.clone(),
                    _ => return Err("Expected identifier after 'ts'".to_string()),
                };

                self.expect(&Token::Pmo)?;
                let expr = self.parse_binary_expression(0)?;
                self.expect(&Token::Semicolon)?;

                Ok(Statement::Declaration(name, expr))
            }
            Some(Token::Hawk) => {
                self.advance();

                let name = match self.advance() {
                    Some(Token::Identifier(id)) => id.clone(),
                    _ => return Err("Expected function name after 'hawk'".to_string()),
                };

                self.expect(&Token::LParen)?;

                let mut params = Vec::new();

                if let Some(Token::RParen) = self.peek() {
                    self.advance();
                } else {
                    loop {
                        match self.advance() {
                            Some(Token::Identifier(id)) => params.push(id.clone()),
                            _ => return Err("Expected parameter name".to_string()),
                        }

                        match self.peek() {
                            Some(Token::Comma) => {
                                self.advance();
                            }
                            Some(Token::RParen) => {
                                self.advance();
                                break;
                            }
                            _ => return Err("Expected ',' or ')' in function parameters".to_string()),
                        }
                    }
                }

                self.expect(&Token::Tuah)?;
                let body = self.parse_binary_expression(0)?;
                self.expect(&Token::Semicolon)?;

                Ok(Statement::Function(name, params, body))
            }
            Some(Token::Goon) => {
                self.advance();

                let condition = if let Some(Token::LParen) = self.peek() {
                    self.advance();
                    let expr = self.parse_binary_expression(0)?;
                    self.expect(&Token::RParen)?;
                    Some(expr)
                } else {
                    None
                };

                let mut body = Vec::new();

                while let Some(token) = self.peek() {
                    if matches!(token, Token::Edge) {
                        self.advance();
                        break;
                    }

                    body.push(self.parse_statement()?);
                }

                Ok(Statement::Loop(condition, body))
            }
            Some(Token::Sybau) => {
                self.advance();
                self.expect(&Token::Semicolon)?;
                Ok(Statement::Break)
            }
            _ => {
                let expr = self.parse_binary_expression(0)?;
                self.expect(&Token::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }
}

struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, (Vec<String>, Expression)>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(val) => Ok(val.clone()),
            Expression::Identifier(name) => {
                match self.variables.get(name) {
                    Some(val) => Ok(val.clone()),
                    None => Err(format!("Undefined variable: {}", name)),
                }
            }
            Expression::Binary(left, op, right) => {
                let left_val = self.eval_expression(left)?;
                let right_val = self.eval_expression(right)?;

                match (left_val.clone(), op.as_str(), right_val.clone()) {
                    (Value::Number(l), "+", Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), "-", Value::Number(r)) => Ok(Value::Number(l - r)),
                    (Value::Number(l), "*", Value::Number(r)) => Ok(Value::Number(l * r)),
                    (Value::Number(l), "/", Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    (Value::String(l), "+", Value::String(r)) => Ok(Value::String(l + &r)),
                    (Value::String(l), "+", Value::Number(r)) => Ok(Value::String(l + &r.to_string())),
                    (Value::Number(l), "+", Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
                    (Value::Number(l), "==", Value::Number(r)) => Ok(Value::Number(if l == r { 1.0 } else { 0.0 })),
                    (Value::Number(l), "!=", Value::Number(r)) => Ok(Value::Number(if l != r { 1.0 } else { 0.0 })),
                    (Value::Number(l), "<", Value::Number(r)) => Ok(Value::Number(if l < r { 1.0 } else { 0.0 })),
                    (Value::Number(l), ">", Value::Number(r)) => Ok(Value::Number(if l > r { 1.0 } else { 0.0 })),
                    (Value::Number(l), "<=", Value::Number(r)) => Ok(Value::Number(if l <= r { 1.0 } else { 0.0 })),
                    (Value::Number(l), ">=", Value::Number(r)) => Ok(Value::Number(if l >= r { 1.0 } else { 0.0 })),
                    // Array access
                    (Value::Array(arr), "[]", Value::Number(idx)) => {
                        let idx = idx as usize;
                        if idx < arr.len() {
                            Ok(arr[idx].clone())
                        } else {
                            Err(format!("Array index out of bounds: {}", idx))
                        }
                    },
                    _ => Err(format!("Invalid operation: {:?} {} {:?}", left_val, op, right_val)),
                }
            }
            Expression::FunctionCall(name, args) => {
                if let Some((params, body)) = self.functions.get(name).cloned() {
                    if args.len() != params.len() {
                        return Err(format!("Function {} expected {} arguments, got {}", name, params.len(), args.len()));
                    }

                    // Evaluate arguments
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(self.eval_expression(arg)?);
                    }

                    // Create a new scope with the parameters bound to the argument values
                    let old_variables = self.variables.clone();

                    for (param, arg) in params.iter().zip(arg_values) {
                        self.variables.insert(param.clone(), arg);
                    }

                    // Evaluate the function body
                    let result = self.eval_expression(&body);

                    // Restore the old scope
                    self.variables = old_variables;

                    result
                } else {
                    Err(format!("Undefined function: {}", name))
                }
            }
            Expression::Array(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.eval_expression(element)?);
                }
                Ok(Value::Array(values))
            }
            Expression::Input => {
                let mut input = String::new();
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();

                // Try to parse as a number first
                if let Ok(n) = input.trim().parse::<f64>() {
                    Ok(Value::Number(n))
                } else {
                    Ok(Value::String(input.trim().to_string()))
                }
            }
        }
    }

    fn execute_statement(&mut self, stmt: &Statement) -> Result<Option<Value>, String> {
        match stmt {
            Statement::Declaration(name, expr) => {
                let value = self.eval_expression(expr)?;
                self.variables.insert(name.clone(), value);
                Ok(None)
            }
            Statement::Print(expr) => {
                let value = self.eval_expression(expr)?;
                match value {
                    Value::String(s) => println!("{}", s),
                    _ => println!("{}", value),
                }
                Ok(None)
            }
            Statement::Function(name, params, body) => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(None)
            }
            Statement::Loop(condition, body) => {
                match condition {
                    Some(expr) => {
                        // For loop (goon(n))
                        let limit = match self.eval_expression(expr)? {
                            Value::Number(n) => n as i32,
                            _ => return Err("Loop condition must evaluate to a number".to_string()),
                        };

                        for _ in 0..limit {
                            for statement in body {
                                if let Some(Value::String(s)) = self.execute_statement(statement)? {
                                    if s == "break" {
                                        return Ok(None);
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        // Infinite loop (goon)
                        loop {
                            for statement in body {
                                if let Some(Value::String(s)) = self.execute_statement(statement)? {
                                    if s == "break" {
                                        return Ok(None);
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(None)
            }
            Statement::Break => {
                Ok(Some(Value::String("break".to_string())))
            }
            Statement::Expression(expr) => {
                self.eval_expression(expr)?;
                Ok(None)
            }
        }
    }

    fn execute_program(&mut self, statements: &[Statement]) -> Result<(), String> {
        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <script.paijorot>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    // Lexical analysis
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // Parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(stmts) => stmts,
        Err(e) => {
            println!("Parse error: {}", e);
            process::exit(1);
        }
    };

    // Interpretation
    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.execute_program(&program) {
        println!("Runtime error: {}", e);
        process::exit(1);
    }
}
