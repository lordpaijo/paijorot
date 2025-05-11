use std::collections::HashMap;
use crate::token::Literal;

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Function(Function),
    Array(Vec<Value>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<crate::parser::Expr>,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Literal(lit) => match lit {
                Literal::String(s) => s.clone(),
                Literal::Number(n) => n.to_string(),
                Literal::Boolean(b) => b.to_string(),
                Literal::Nil => "nil".to_string(),
            },
            Value::Function(f) => format!("<function {}>", f.name),
            Value::Array(elements) => {
                let elements_str: Vec<String> = elements.iter()
                    .map(|e| e.to_string())
                    .collect();
                format!("[{}]", elements_str.join(", "))
            }
        }
    }
}

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned()
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }
}
