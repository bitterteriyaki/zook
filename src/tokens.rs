#[derive(Clone)]
pub enum TokenType {
    Exit,
    Integer,
    Semicolon,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: Option<String>,
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = content.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            p if p.is_alphabetic() => {
                let mut value = String::new();
                value.push(p);

                while let Some(q) = iter.peek() {
                    if !q.is_alphabetic() {
                        break;
                    }

                    value.push(q.clone());
                    iter.next();
                }

                match value.as_str() {
                    "exit" => tokens.push(Token { kind: TokenType::Exit, value: None }),
                    _ => panic!("Error: Unexpected keyword: {}", value),
                }
            },
            p if p.is_numeric() => {
                let mut value = String::new();
                value.push(p);

                while let Some(q) = iter.peek() {
                    if !q.is_numeric() {
                        break;
                    }

                    value.push(q.clone());
                    iter.next();
                }

                tokens.push(Token {
                    kind: TokenType::Integer,
                    value: Some(value),
                });
            },
            p if p.is_whitespace() => continue,
            ';' => tokens.push(Token { kind: TokenType::Semicolon, value: None }),
            _ => panic!("Error: Unexpected character: {}", c),
        }
    }

    tokens
}
