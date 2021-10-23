use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub value: f64,
}

pub fn parse_float_literal(p: &mut Parser) -> Option<Node> {
    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<f64>()
        .unwrap();

    let exp = Expression::Float(Float { value });

    Some(Node::Expression(exp))
}

pub fn parse_minus_float(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<f64>()
        .unwrap();

    Some(Node::Expression(Expression::Float(Float { value: -value })))
}
