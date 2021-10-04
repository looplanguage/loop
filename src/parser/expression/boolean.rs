use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub(crate) value: bool,
}

pub fn parse_boolean(p: &mut Parser) -> Option<Node> {
    Some(Node::Expression(Expression::Boolean(Boolean {
        value: p.lexer.current_token.clone().unwrap().literal == "true",
    })))
}

pub fn parse_inverted_boolean(p: &mut Parser) -> Option<Node> {
    p.lexer.next();

    Some(Node::Expression(Expression::Boolean(Boolean {
        value: p.lexer.current_token.clone().unwrap().literal != "true",
    })))
}
