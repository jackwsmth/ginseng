#[derive(Debug)]
enum TokenType {
    INT,
    ADD,
    SUB,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    token_value: String,
}

fn main() {
    let tokens = tokenize("42 + 123");
    println!("{:?}", tokens);
}

fn tokenize(expr: &str) -> Vec<&str> {
    let words: Vec<&str> = expr.split_whitespace().collect();
    words
}
