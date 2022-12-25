mod lexer;
mod builtin;

use lexer::Lexer;

fn main() {
    // Lexing
    let mut lexer = Lexer::new(String::from("src/test.lf"));
    let mut tokens = lexer.lex();
    dbg!(lexer);
}
