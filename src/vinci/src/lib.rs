use crate::ast::AST;
use crate::lexer::token::Token;
use crate::parser::Parser;
use logos::Logos;

pub mod ast;
mod lexer;
mod parser;
pub mod types;

pub fn parse(arc: &str) -> AST {
    let lexer = Token::lexer(arc);
    let mut parser = Parser::new(lexer);

    parser.parse()
}
