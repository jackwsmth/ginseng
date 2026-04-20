#[derive(Debug)]
enum Token {
    Int(u128),
    ADD,
    SUB,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

fn main() {
    let tokens = tokenize("42 + 123");
    println!("{:?}", tokens);
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_number = String::new();

    for c in expr.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                tokens.push(build_token(TokenType::INT, current_number.clone()));
                current_number.clear();
            }
        }

        match c {
            ' ' => continue,
            '(' => tokens.push(build_token(TokenType::LPAREN, String::from("("))),
            ')' => tokens.push(build_token(TokenType::RPAREN, String::from(")"))),
            '+' => tokens.push(build_token(TokenType::ADD, String::from("+"))),
            '-' => tokens.push(build_token(TokenType::SUB, String::from("-"))),
            '*' => tokens.push(build_token(TokenType::MUL, String::from("*"))),
            '/' => tokens.push(build_token(TokenType::DIV, String::from("/"))),
            _ => continue,
        }
    }

    if !current_number.is_empty() {
        tokens.push(build_token(TokenType::INT, current_number));
    }

    tokens
}

fn build_token(token_type: TokenType, token_value: String) -> Token {
    Token {
        token_type,
        token_value,
    }
}
