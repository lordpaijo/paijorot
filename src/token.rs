#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Semicolon,

    // Operators
    Plus, Minus, Star, Slash, Modulo,
    Equal, NotEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number, Boolean,

    // Keywords
    Yap,      // println!()
    Ts,       // let
    Pmo,      // =
    Gyat,     // array
    Hawk,     // fn
    Tuah,     // return
    Goon,     // loop/for
    Edge,     // end of loop
    Yeet,     // read input
    Sybau,    // break
    Yo,       // if
    Gurt,     // else

    EOF
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
