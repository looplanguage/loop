use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Null {}

pub fn parse_expression_null(p: &mut Parser) -> Option<Node> {
    Some(Node::Expression(Expression::Null(Null {})))
}
