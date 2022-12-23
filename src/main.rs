mod lexer;
mod parser;
mod builtin;

use lexer::Lexer;
use parser::Parser;

fn main() {
    // Lexing
    let mut lexer = Lexer::new(String::from("src/test.lf"));
    let mut tokens = lexer.lex();
    dbg!(lexer);
    
    // Parsing
    let mut parser = Parser::new(tokens);
    let mut node = parser.parse();
    dbg!(node);
}
