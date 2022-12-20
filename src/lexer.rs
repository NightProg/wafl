use std::fs;

#[derive(Debug)]
pub enum Macro{
    Defun,
    Vec,
    Tup,
    While,
    If,
    Else,
    Let
}

#[derive(Debug)]
pub enum Builtin {}

#[derive(Debug)]
pub enum LType {
    LParen, // (
    RParen, // )
    LBracket, // <
    RBracket, // >
    Quote, // '
    Macro(Macro),
    Ident(String),
    Str(String),
    Integer(i32),
    Real(f64),
    Builtin(Builtin)
}
impl LType {
    pub fn get_type(&self) -> String {
        match self {
            Self::LParen => String::from("Opening Parenthese"),
            Self::RParen => String::from("Closing Parenthese"),
            Self::LBracket => String::from("Opening Chevron"),
            Self::RBracket => String::from("Closing Chevron"),
            Self::Quote => String::from("Quote"),
            Self::Macro(_) => String::from("Macro"),
            Self::Ident(_) => String::from("Identifier"),
            Self::Str(_) => String::from("String"),
            Self::Integer(_) => String::from("Integer"),
            Self::Real(_) => String::from("Real"),
            _ => String::from("Keyword")
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub ltype: LType
}

impl Token {
    pub fn new(ltype: LType) -> Self {
        Self { ltype }
    }
}

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    line: usize,
    start: usize,
    current: usize // is column
}

impl Lexer {
    pub fn new(file: String) -> Self {
        Self {
            input: fs::read_to_string(file).expect("Cannot read file."),
            output: vec![],
            line: 0,
            start: 0,
            current: 0
        }
    }

    pub fn debug(&self) -> String {
        format!(
            "output: {:?}, line: {}, start: {}, current_index: {}, current_value: {:?}",
            self.output,
            self.line,
            self.start,
            self.current,
            self.input.chars().nth(self.current)
        )
    }
}