use std::{iter::Peekable, slice::Iter};
use crate::tokens::{Token, TokenType};

pub struct NodeExpr {
    pub integer: Token,
}

pub struct NodeExit {
    pub expr: NodeExpr,
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn get_root(self) -> NodeExit {
        let mut iter = self.tokens.iter().peekable();

        while let Some(token) = iter.next() {
            match token.kind {
                TokenType::Exit => {
                    let expr = Parser::parse_expr(&mut iter);
                    let semi = iter.next()
                        .expect("Error: Unexpected end of file");

                    match semi.kind {
                        TokenType::Semicolon => return NodeExit { expr },
                        _ => panic!("Error: Unexpected token: {:?}", semi.value),
                    }
                },
                _ => panic!("Error: Unexpected token: {:?}", token.value),
            }
        }

        panic!("Error: Unexpected end of file");
    }

    fn parse_expr(iter: &mut Peekable<Iter<'_, Token>>) -> NodeExpr {
        let token = iter.next().expect("Error: Unexpected end of file");

        match token.kind {
            TokenType::Integer => NodeExpr { integer: token.clone() },
            _ => panic!("Error: Unexpected token: {:?}", token.value),
        }
    }
}
