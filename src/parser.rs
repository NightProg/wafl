use crate::lexer::{
    LType,
    Lexer
};

pub enum Node {
    Add(f64, f64),
    Sub(f64, f64),
    Div(f64, f64),
    Mod(f64, f64),
    Mul(f64, f64),
    Defun(String, Vec<String>, Vec<LType>), // name, args, do
    Vec(Vec<LType>), // args
    Tup(Vec<LType>), // args
    Let(String, LType), // name, val
    While(Vec<LType>, Vec<LType>), // cond, do
    If(Vec<LType>, Vec<LType>), // cond, do
    Else(Vec<LType>), // do

    Builtin(String, Vec<LType>) // name, args
}

impl Node {
    pub fn get_type(&self) -> String {
        match self {
            Node::Add(_, _) | Node::Sub(_, _) | Node::Div(_, _) | Node::Mod(_, _) | Node::Mul(_, _) => String::from("Operation"),
            Node::Defun(_, _, _,) => String::from("Defun"),
            Node::Vec(_) => String::from("Vector"),
            Node::Tup(_) => String::from("Tuple"),
            Node::Let(_, _) => String::from("Let"),
            Node::While(_, _) => String::from("While"),
            Node::If(_, _) => String::from("If"),
            Node::Else(_) => String::from("Else"),
            Node::Builtin(_, _) => String::from("Buildin"),
            _ => String::from("47 4e 55")
        }
    }
}

pub struct Parser {
    output: Vec<Node>,
    input: Vec<LType>,
    start: usize,
    current: usize,
    line: usize
}