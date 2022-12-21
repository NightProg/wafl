use std::fs;

#[derive(Debug, PartialEq)]
pub enum Macro {
    Defun,
    Vec,
    Tup,
    While,
    If,
    Else,
    Let
}

#[derive(Debug, PartialEq)]
pub enum Builtin {}

#[derive(Debug, PartialEq)]
pub enum LType {
    LParen, // (
    RParen, // )
    LBracket, // <
    RBracket, // >
    Quote, // "
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

#[derive(Debug, PartialEq)]
pub struct Token(LType);

impl Token {
    pub fn new(ltype: LType) -> Self {
        Self ( ltype )
    }
}

pub struct Lexer {
    pub input: String,
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
            line: 1,
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

    pub fn is_eof(&self) -> bool {
        self.current >= self.input.chars().count()
    }

    pub fn peek(&self) -> char {
        self.input.chars().nth(self.current).expect("Hmm.")
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    pub fn lex_one(&mut self) {
        let cc = self.advance();

        match cc {
            '>' => self.output.push(Token::new(LType::RBracket)),
            '<' => self.output.push(Token::new(LType::LBracket)),
            '(' => self.output.push(Token::new(LType::LParen)),
            ')' => self.output.push(Token::new(LType::RParen)),
            '"' => self.output.push(Token::new(LType::Quote)),
            '\n' => self.line += 1,
            x => if *self.output.last().unwrap() == Token(LType::Quote) {
                    self.string();
                } else if x.is_numeric() {
                    self.number();
                } else {
                    self.identifier();
                }
        }
    }

    pub fn string(&mut self) {
        self.start = self.current;

        while !self.is_eof() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_eof() {
            eprintln!("Unlimited string.");
        }

        let str = self.input[self.start - 1..self.current].to_owned();
        println!("{}", str);

        self.output.push(Token::new(LType::Str(str)));
    }

    pub fn number(&self) {}

    pub fn identifier(&self) {}

    pub fn lex(&mut self) {
        while !self.is_eof() {
            self.lex_one();
        }
    }
}