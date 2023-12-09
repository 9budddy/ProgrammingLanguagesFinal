#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use crate::token::Token;

pub struct Lexer {
    tokens : Vec<Token>,
    idx : usize,
    current_token: Token,
}

impl Lexer {

    pub fn new(tokens : Vec<Token>) -> Lexer {
        let mut tokens = tokens;
        tokens.push(Token::EOI);
        let idx  = 0;
        let current_token = tokens[0].clone();
        Lexer {
            tokens,
            idx,
            current_token
        }
    }

    pub fn current(&self) -> Token {
        self.current_token.clone()
    }

    pub fn advance(&mut self) {
        if self.idx < self.tokens.len() - 1 { // always: tokens.len() > 0
            self.idx += 1;
            self.current_token = self.tokens[self.idx].clone();
        }
    }

}