use crate::token::{Token, TokenType, Literal};
use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("yap".to_string(), TokenType::Yap);
        keywords.insert("ts".to_string(), TokenType::Ts);
        keywords.insert("pmo".to_string(), TokenType::Pmo);
        keywords.insert("gyat".to_string(), TokenType::Gyat);
        keywords.insert("gyatt".to_string(), TokenType::Gyat); // Alias for gyat
        keywords.insert("hawk".to_string(), TokenType::Hawk);
        keywords.insert("tuah".to_string(), TokenType::Tuah);
        keywords.insert("goon".to_string(), TokenType::Goon);
        keywords.insert("edge".to_string(), TokenType::Edge);
        keywords.insert("yeet".to_string(), TokenType::Yeet);
        keywords.insert("sybau".to_string(), TokenType::Sybau);
        keywords.insert("yo".to_string(), TokenType::Yo);
        keywords.insert("gurt".to_string(), TokenType::Gurt);

        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            self.line,
        ));

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_char('/') {
                    // Comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            '%' => self.add_token(TokenType::Modulo),
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Equal);
                } else {
                    return Err(format!("Unexpected character '=' at line {}", self.line));
                }
            },
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::NotEqual);
                } else {
                    return Err(format!("Unexpected character '!' at line {}", self.line));
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            },
            ' ' | '\r' | '\t' => {}, // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                if self.is_digit(c) {
                    self.number()?;
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(format!("Unexpected character '{}' at line {}", c, self.line));
                }
            }
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].iter().collect::<String>();

        let token_type = self.keywords.get(&text).cloned().unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn number(&mut self) -> Result<(), String> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for decimal point
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let parsed_value = value.parse::<f64>().map_err(|_| {
            format!("Failed to parse number at line {}", self.line)
        })?;

        self.add_token_literal(TokenType::Number, Some(Literal::Number(parsed_value)));
        Ok(())
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string at line {}", self.line));
        }

        // Consume the closing "
        self.advance();

        // Trim the surrounding quotes
        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        // Process escape sequences
        let value = self.process_escape_sequences(value)?;

        self.add_token_literal(TokenType::String, Some(Literal::String(value)));
        Ok(())
    }

    fn process_escape_sequences(&self, input: String) -> Result<String, String> {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' && chars.peek().is_some() {
                match chars.next().unwrap() {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    c => return Err(format!("Invalid escape sequence \\{} at line {}", c, self.line)),
                }
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }
}
