use std::env::args;
use std::io::{Read, Result, Write};
use std::process::{exit, Command};
use std::fs::File;

#[derive(Debug)]
enum TokenType {
    Return,
    Integer,
    Semicolon,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: Option<String>,
}

fn fatal(msg: &str) {
    eprintln!("\x1b[31m[zook fatal error]\x1b[0m");
    eprintln!("{}", msg);
    exit(1);
}

fn tokenize(content: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = content.chars().peekable();

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
                    "return" => tokens.push(Token {
                        kind: TokenType::Return,
                        value: None,
                    }),
                    _ => fatal(&format!("Unexpected keyword: {}", value)),
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
            _ => fatal(&format!("Unexpected character: {}", c)),
        }
    }

    tokens
}

fn to_asm(tokens: &Vec<Token>) -> String {
    let mut output = String::new();
    let mut iter = tokens.iter().peekable();
    
    output.push_str("global _start\n_start:\n");
    
    // This is the worst code I've ever written, I'm sorry. I'll refactor it
    // later.
    while let Some(token) = iter.next() {
        match token.kind {
            TokenType::Return => {
                if let None = iter.peek() {
                    fatal("Unexpected end of file");
                }

                let next = iter.next().unwrap();

                match next.kind {
                    TokenType::Integer => {
                        if let None = iter.peek() {
                            fatal("Unexpected end of file");
                        }

                        let next2 = iter.next().unwrap();

                        match next2.kind {
                            TokenType::Semicolon => {
                                output.push_str("  mov rax, 60\n");
                                output.push_str("  mov rdi, ");
                                output.push_str(&next.value.as_ref().unwrap());
                                output.push_str("\n");
                                output.push_str("  syscall\n");
                            },
                            _ => fatal("Unexpected token"),
                        }
                    },
                    _ => fatal("Unexpected token"),
                }
            },
            _ => fatal("Unexpected token"),
        }
    }

    output
}

fn main() -> Result<()> {
    // Maybe parse the command line with clap or structopt and use colored crate
    // to colorize the output properly.
    let args: Vec<String> = args().collect();
    let executable = &args[0];

    if args.len() < 2 {
        fatal(&format!("Usage: {} <file>\n", executable));
    }

    // Show the correct error message if the file does not exist using the
    // `fatal` function created above.
    let mut file = File::open(&args[1])?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let tokens = tokenize(&content);
    let asm = to_asm(&tokens);

    let mut file = File::create("out.asm")?;
    file.write_all(asm.as_bytes())?;

    Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("out.asm")
        .output()?;

    Command::new("ld")
        .arg("out.o")
        .arg("-o")
        .arg("out")
        .output()?;

    Ok(())
}
