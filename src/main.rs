mod lexer;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("src/test.lf"));
    lexer.lex();
    let debug_lexer = lexer.debug();
    println!("{:?}", debug_lexer);
}
