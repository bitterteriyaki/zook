#[derive(Debug)]
#[derive(Clone)]
pub enum TokenType {
    Exit,
    Integer,
    Semicolon,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    content: String,
}

impl Tokenizer {
    pub fn new(content: &str) -> Self {
        Tokenizer { content: content.to_string() }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = self.content.chars().peekable();
    
        while let Some(c) = iter.next() {
            match c {
                t if t.is_alphabetic() => {
                    let mut value = String::new();
                    value.push(t);
    
                    while let Some(r) = iter.peek() {
                        if !r.is_alphabetic() {
                            break;
                        }
    
                        value.push(r.clone());
                        iter.next();
                    }
    
                    match value.as_str() {
                        "exit" => tokens.push(Token {
                            kind: TokenType::Exit,
                            value: None,
                        }),
                        _ => panic!("Unexpected keyword: {}", value),
                    }
                },
                t if t.is_numeric() => {
                    let mut value = String::new();
                    value.push(t);
    
                    while let Some(r) = iter.peek() {
                        if !r.is_numeric() {
                            break;
                        }
    
                        value.push(r.clone());
                        iter.next();
                    }
    
                    tokens.push(Token {
                        kind: TokenType::Integer,
                        value: Some(value),
                    });
                },
                ';' => tokens.push(Token {
                    kind: TokenType::Semicolon,
                    value: None,
                }),
                t if t.is_whitespace() => continue,
                _ => panic!("Unexpected character: {}", c),
            }
        }
    
        tokens
    }
}
