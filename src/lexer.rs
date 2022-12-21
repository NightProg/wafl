use std::fs;

const BUILTINS: &[&str] = &[];
const MACROS: &[&str] = &["defun", "vec", "tup", "if", "else", "while", "let"];

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
    current: usize,
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
            ' ' => {}
            '>' => self.add_token(LType::RBracket),
            '<' => self.add_token(LType::LBracket),
            '(' => self.add_token(LType::LParen),
            ')' => self.add_token(LType::RParen),
            '"' => self.add_token(LType::Quote),
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

    pub fn add_token(&mut self, ltype: LType) {
        self.output.push(Token(ltype));
    }

    pub fn string(&mut self) {
        'outer: while !self.is_eof() {
            match self.peek() {
                '"' => break 'outer,
                '\n' => self.line += 1,
                _ => (),
            }
            self.advance();
        }
        if self.is_eof() {
            eprintln!("Unlimited string.");
        }

        let str = self.input[self.start..self.current].to_owned();
        println!("Str: '{}'", str);
        self.current += 1; // Spell of quotation marks
        self.add_token(LType::Str(str));
    }

    pub fn number(&self) {}

    pub fn identifier(&mut self) {
        let stop = vec!['(',')', '\n', ' ', '"'];
        while !self.is_eof() && !stop.contains(&self.peek()) { self.advance(); }
        let ident = self.input[self.start..self.current].to_string();
        
        match &ident as &str {
            // Macro
            "defun" => self.add_token(LType::Macro(Macro::Defun)),
            "vec" => self.add_token(LType::Macro(Macro::Vec)),
            "tup" => self.add_token(LType::Macro(Macro::Tup)),
            "if" => self.add_token(LType::Macro(Macro::If)),
            "else" => self.add_token(LType::Macro(Macro::Else)),
            "while" => self.add_token(LType::Macro(Macro::While)),
            "let" => self.add_token(LType::Macro(Macro::Let)),

            //Builtin
            // ...
            
            _ => self.add_token(LType::Ident(ident))
        }
    }

    pub fn lex(&mut self) {
        while !self.is_eof() {
            self.lex_one();
            self.start = self.current;
        }
    }
}