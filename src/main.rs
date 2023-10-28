use std::env;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::process::Command;

#[derive(Debug, PartialEq)]
enum TokenType {
    Exit,
    IntLiteral,
    Semicolon,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_alphabetic() {
            let mut value = String::new();
            value.push(c);

            while let Some(c) = chars.peek() {
                if !c.is_alphabetic() {
                    break;
                }

                value.push(chars.next().unwrap());
            }

            match value.as_str() {
                "exit" => {
                    tokens.push(Token {
                        token_type: TokenType::Exit,
                        value: None,
                    });
                }
                _ => {
                    println!("Unexpected keyword: {}", value);
                    break;
                }
            }
        } else if c.is_numeric() {
            let mut value = String::new();
            value.push(c);

            while let Some(c) = chars.peek() {
                if c.is_numeric() {
                    value.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(Token {
                token_type: TokenType::IntLiteral,
                value: Some(value),
            });
        } else if c == ';' {
            tokens.push(Token {
                token_type: TokenType::Semicolon,
                value: None,
            });
        } else if c.is_whitespace() {
            continue;
        } else {
            panic!("Unexpected character: {}", c)
        }
    }

    return tokens;
}

fn to_asm(tokens: Vec<Token>) -> String {
    let mut output: String = "global _main\n_main:\n".to_string();

    for (pos, token) in tokens.iter().enumerate() {
        if token.token_type == TokenType::Exit {
            if pos + 1 < tokens.len() && tokens[pos + 1].token_type == TokenType::IntLiteral {
                if pos + 2 < tokens.len() && tokens[pos + 2].token_type == TokenType::Semicolon {
                    output.push_str("    mov rax, 60\n");
                    output.push_str("    mov rdi, ");
                    output.push_str(tokens[pos + 1].value.as_ref().unwrap());
                    output.push_str("\n");
                    output.push_str("    syscall\n");
                }
            }
        }
    }

    return output;
}

fn main() -> Result<()> {
    let filename: String = env::args()
        .skip(1)
        .collect();
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    let tokens: Vec<Token> = tokenize(content);
    let asm: String = to_asm(tokens);

    let mut file = File::create("out.asm")?;
    file.write_all(asm.as_bytes())?;

    Ok(())
}
