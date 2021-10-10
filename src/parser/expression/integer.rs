use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64,
}

pub fn parse_integer_literal(p: &mut Parser) -> Option<Node> {
    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<i64>()
        .unwrap();

    Some(Node::Expression(Expression::Integer(Integer { value })))
}

pub fn parse_minus_integer(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<i64>()
        .unwrap();

    Some(Node::Expression(Expression::Integer(Integer {
        value: -value,
    })))
}
