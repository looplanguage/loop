use crate::parser::exception::SyntaxException;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Boolean {
    pub(crate) value: bool,
}

pub fn parse_boolean(p: &mut Parser) -> Result<Node, SyntaxException> {
    Ok(Node::Expression(Expression::Boolean(Boolean {
        value: p.lexer.get_current_token().unwrap().literal == "true",
    })))
}

pub fn parse_inverted_boolean(p: &mut Parser) -> Result<Node, SyntaxException> {
    p.lexer.next_token();

    Ok(Node::Expression(Expression::Boolean(Boolean {
        value: p.lexer.get_current_token().unwrap().literal != "true",
    })))
}
