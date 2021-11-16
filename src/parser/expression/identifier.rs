use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub(crate) value: String,
}

pub fn parse_identifier(p: &mut Parser) -> Option<Node> {
    Some(Node::Expression(Expression::Identifier(Identifier {
        value: p.lexer.get_current_token().unwrap().literal.clone(),
    })))
}
