mod lexer;

use lexer::Lexer;

fn main() {
    let lexer = Lexer::new(String::from("src/test.lf"));
    let debug_lexer = lexer.debug();
    println!("{:?}", debug_lexer);
}
