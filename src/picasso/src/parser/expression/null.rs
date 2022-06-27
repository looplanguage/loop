use crate::parser::exception::SyntaxException;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Null {}

pub fn parse_expression_null(_p: &mut Parser) -> Result<Node, SyntaxException> {
    Ok(Node::Expression(Expression::Null(Null {})))
}
