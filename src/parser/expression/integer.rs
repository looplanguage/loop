use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i32,
}

pub fn parse_integer_literal(p: &mut Parser) -> Option<Node> {
    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<i32>()
        .unwrap();

    Some(Node::Expression(Expression::Integer(Integer { value })))
}
