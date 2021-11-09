use crate::parser::Parser;
use crate::parser::program::Node;

pub fn parse_comment(p: &mut Parser) -> Option<Node>{
    println!("input: {}", p.lexer.current_token.as_ref().unwrap().literal);
    None
}