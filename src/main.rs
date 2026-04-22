mod lexer;
use lexer::tokenize;

fn main() {
    let tokens = tokenize("42 + 123");
    println!("{:?}", tokens);
}
