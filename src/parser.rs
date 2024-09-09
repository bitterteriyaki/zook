use std::{iter::Peekable, slice::Iter};

use crate::tokens::{Token, TokenType};

pub struct NodeExpr {
    pub integer: Token
}

pub struct NodeExit {
    pub expr: NodeExpr,
}

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    fn parse_expr(self, iter: &mut Peekable<Iter<'_, Token>>) -> NodeExpr {
        if let None = iter.peek() {
            panic!("Unexpected end of file");
        }

        let token = iter.next().unwrap();

        match token.kind {
            TokenType::Integer => (),
            _ => panic!("Unexpected token: {:?}", token),
            
        }

        NodeExpr { integer: token.clone() }
    }

    pub fn parse(&self) -> NodeExit {
        let mut iter = self.tokens.iter().peekable();

        while let Some(token) = iter.next() {
            match token.kind {
                TokenType::Exit => {
                    let expr = self.clone().parse_expr(&mut iter);

                    if let None = iter.peek() {
                        panic!("Unexpected end of file");
                    }

                    let semi = iter.next().unwrap();

                    match semi.kind {
                        TokenType::Semicolon => (),
                        _ => panic!("Unexpected token: {:?}", semi),
                    }

                    return NodeExit { expr };
                },
                _ => panic!("Unexpected token: {:?}", token),
            }
        }

        NodeExit {
            expr: NodeExpr {
                integer: Token {
                    kind: crate::tokens::TokenType::Integer,
                    value: Some("0".to_string()),
                },
            },
        }
    }
}
