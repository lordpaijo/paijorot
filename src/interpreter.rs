use crate::parser::{Expr, Stmt};
use crate::token::{TokenType, Literal};
use crate::environment::{Environment, Value, Function};
use std::io::{self, Write, BufRead};

pub struct Interpreter<'a> {
    environment: &'a mut Environment,
    in_loop: bool,
    should_break: bool,
}

impl<'a> Interpreter<'a> {
    pub fn new(environment: &'a mut Environment) -> Self {
        Interpreter {
            environment,
            in_loop: false,
            should_break: false,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute(&stmt)?;

            if self.should_break {
                return Err("'sybau' statement outside of a loop.".to_string());
            }
        }

        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            },
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value.to_string());
                Ok(())
            },
            Stmt::Var(name, initializer) => {
                let value = if let Some(expr) = initializer {
                    self.evaluate(expr)?
                } else {
                    Value::Literal(Literal::Nil)
                };

                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            },
            Stmt::If(condition, then_branch, else_branch) => {
                let condition_value = self.evaluate(condition)?;

                if self.is_truthy(&condition_value) {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }

                Ok(())
            },
            Stmt::Loop(condition, body) => {
                let previous_in_loop = self.in_loop;
                self.in_loop = true;

                // If a condition is present, this is a goon(n) loop
                if let Some(count_expr) = condition {
                    let count_value = self.evaluate(count_expr)?;

                    if let Value::Literal(Literal::Number(n)) = count_value {
                        let iterations = n as i64;

                        for _ in 0..iterations {
                            for stmt in body {
                                self.execute(stmt)?;

                                if self.should_break {
                                    self.should_break = false;
                                    break;
                                }
                            }

                            if self.should_break {
                                self.should_break = false;
                                break;
                            }
                        }
                    } else {
                        return Err("Loop condition must evaluate to a number.".to_string());
                    }
                } else {
                    // Infinite loop (goon)
                    loop {
                        for stmt in body {
                            self.execute(stmt)?;

                            if self.should_break {
                                self.should_break = false;
                                break;
                            }
                        }

                        if self.should_break {
                            self.should_break = false;
                            break;
                        }
                    }
                }

                self.in_loop = previous_in_loop;
                Ok(())
            },
            Stmt::Break => {
                if self.in_loop {
                    self.should_break = true;
                    Ok(())
                } else {
                    Err("'sybau' statement outside of a loop.".to_string())
                }
            },
            Stmt::Function(name, params, body) => {
                let function = Function {
                    name: name.lexeme.clone(),
                    params: params.iter().map(|param| param.lexeme.clone()).collect(),
                    body: Box::new(body.clone()),
                };

                self.environment.define(
                    name.lexeme.clone(),
                    Value::Function(function),
                );

                Ok(())
            },
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(literal) => {
                // Special case for yeet (input)
                if let Literal::String(s) = literal {
                    if s == "__YEET__" {
                        return self.handle_input();
                    }
                }

                Ok(Value::Literal(literal.clone()))
            },
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Variable(name) => {
                match self.environment.get(&name.lexeme) {
                    Some(value) => Ok(value),
                    None => Err(format!("Undefined variable '{}'.", name.lexeme)),
                }
            },
            Expr::Binary(left, operator, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Plus => self.add(&left_val, &right_val),
                    TokenType::Minus => self.subtract(&left_val, &right_val),
                    TokenType::Star => self.multiply(&left_val, &right_val),
                    TokenType::Slash => self.divide(&left_val, &right_val),
                    TokenType::Modulo => self.modulo(&left_val, &right_val),
                    TokenType::Greater => self.greater(&left_val, &right_val),
                    TokenType::GreaterEqual => self.greater_equal(&left_val, &right_val),
                    TokenType::Less => self.less(&left_val, &right_val),
                    TokenType::LessEqual => self.less_equal(&left_val, &right_val),
                    TokenType::Equal => self.equal(&left_val, &right_val),
                    TokenType::NotEqual => self.not_equal(&left_val, &right_val),
                    TokenType::Pmo => {
                        // Handle assignment
                        if let Expr::Variable(var_name) = &**left {
                            self.environment.assign(&var_name.lexeme, right_val.clone())?;
                            Ok(right_val)
                        } else {
                            Err("Invalid assignment target.".to_string())
                        }
                    },
                    _ => Err(format!("Unsupported binary operation: {:?}", operator.token_type)),
                }
            },
            Expr::Array(name, elements) => {
                let mut array_values = Vec::new();

                for element in elements {
                    array_values.push(self.evaluate(element)?);
                }

                let array_value = Value::Array(array_values);
                self.environment.define(name.lexeme.clone(), array_value.clone());

                Ok(array_value)
            },
            Expr::Call(callee, _paren, arguments) => {
                let callee_val = self.evaluate(callee)?;

                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate(arg)?);
                }

                self.call_function(&callee_val, arg_values)
            },
        }
    }

    fn handle_input(&self) -> Result<Value, String> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(_) => {
                // Try to parse as number first
                match input.trim().parse::<f64>() {
                    Ok(n) => Ok(Value::Literal(Literal::Number(n))),
                    Err(_) => Ok(Value::Literal(Literal::String(input.trim().to_string()))),
                }
            },
            Err(_) => Err("Failed to read input.".to_string()),
        }
    }

    fn call_function(&mut self, callee: &Value, arguments: Vec<Value>) -> Result<Value, String> {
        if let Value::Function(function) = callee {
            if function.params.len() != arguments.len() {
                return Err(format!(
                    "Expected {} arguments but got {}.",
                    function.params.len(),
                    arguments.len()
                ));
            }

            // Store arguments in a temporary environment
            let mut temp_env = Environment::new();

            // Define arguments in the temporary environment
            for (param, arg) in function.params.iter().zip(arguments) {
                temp_env.define(param.clone(), arg);
            }

            // Create a new interpreter with the temporary environment
            let mut interpreter = Interpreter {
                environment: &mut temp_env,
                in_loop: false,
                should_break: false,
            };

            // Evaluate the function body
            let result = interpreter.evaluate(&function.body)?;

            Ok(result)
        } else {
            Err("Can only call functions.".to_string())
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Literal(Literal::Nil) => false,
            Value::Literal(Literal::Boolean(b)) => *b,
            _ => true,
        }
    }

    fn add(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Number(a + b)))
            },
            (Value::Literal(Literal::String(a)), Value::Literal(Literal::String(b))) => {
                Ok(Value::Literal(Literal::String(format!("{}{}", a, b))))
            },
            (Value::Literal(Literal::String(a)), b) => {
                Ok(Value::Literal(Literal::String(format!("{}{}", a, b.to_string()))))
            },
            (a, Value::Literal(Literal::String(b))) => {
                Ok(Value::Literal(Literal::String(format!("{}{}", a.to_string(), b))))
            },
            _ => Err("Operands must be numbers or strings.".to_string()),
        }
    }

    fn subtract(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Number(a - b)))
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn multiply(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Number(a * b)))
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn divide(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                if *b == 0.0 {
                    Err("Division by zero.".to_string())
                } else {
                    Ok(Value::Literal(Literal::Number(a / b)))
                }
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn modulo(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                if *b == 0.0 {
                    Err("Modulo by zero.".to_string())
                } else {
                    Ok(Value::Literal(Literal::Number(a % b)))
                }
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn greater(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Boolean(a > b)))
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn greater_equal(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Boolean(a >= b)))
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn less(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Boolean(a < b)))
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn less_equal(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Boolean(a <= b)))
            },
            _ => Err("Operands must be numbers.".to_string()),
        }
    }

    fn equal(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Literal(Literal::Number(a)), Value::Literal(Literal::Number(b))) => {
                Ok(Value::Literal(Literal::Boolean(a == b)))
            },
            (Value::Literal(Literal::String(a)), Value::Literal(Literal::String(b))) => {
                Ok(Value::Literal(Literal::Boolean(a == b)))
            },
            (Value::Literal(Literal::Boolean(a)), Value::Literal(Literal::Boolean(b))) => {
                Ok(Value::Literal(Literal::Boolean(a == b)))
            },
            (Value::Literal(Literal::Nil), Value::Literal(Literal::Nil)) => {
                Ok(Value::Literal(Literal::Boolean(true)))
            },
            _ => Ok(Value::Literal(Literal::Boolean(false))),
        }
    }

    fn not_equal(&self, left: &Value, right: &Value) -> Result<Value, String> {
        let result = self.equal(left, right)?;
        if let Value::Literal(Literal::Boolean(b)) = result {
            Ok(Value::Literal(Literal::Boolean(!b)))
        } else {
            unreachable!()
        }
    }
}
