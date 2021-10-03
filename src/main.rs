extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::lexer::token::{Token, TokenType};

mod lexer;

fn main() {
    let mut l = lexer::build_lexer("var test = 1;");

    let mut current_token: Token = l.next();

    while current_token.token != TokenType::EOF {
        println!("{}: {}", current_token.literal, current_token.token.as_ref());

        current_token = l.next();
    }
}