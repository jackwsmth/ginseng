enum TokenType {
    INT,
    ADD,
    SUB,
    MUL,
    DIV,
    LPAREN,
    RPAREN
}

struct Token {
    token_type: TokenType,
    token_value: String,
}

fn main() {
    let tokens = tokenize("(1 + 2)");
    println!("{:?}", tokens);
}

fn tokenize(expr: &str) -> Vec<&str> {
    let words: Vec<&str> = expr.split_whitespace().collect();
    words
}
