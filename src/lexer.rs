#[derive(Debug)]
pub enum Token {
    Int(u128),
    ADD,
    SUB,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_number = String::new();

    for c in expr.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                let number = current_number.parse::<u128>().unwrap();
                tokens.push(Token::Int(number));
                current_number.clear();
            }
        }

        match c {
            ' ' => continue,
            '(' => tokens.push(Token::LPAREN),
            ')' => tokens.push(Token::RPAREN),
            '+' => tokens.push(Token::ADD),
            '-' => tokens.push(Token::SUB),
            '*' => tokens.push(Token::MUL),
            '/' => tokens.push(Token::DIV),
            _ => continue,
        }
    }

    if !current_number.is_empty() {
        let number = current_number.parse::<u128>().unwrap();
        tokens.push(Token::Int(number));
    }

    tokens
}
