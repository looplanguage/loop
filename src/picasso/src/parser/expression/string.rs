use crate::parser::exception::SyntaxException;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct LoopString {
    pub value: std::string::String,
}

pub fn parse_string_literal(p: &mut Parser) -> Result<Node, SyntaxException> {
    let value = p.lexer.get_current_token().unwrap().literal.clone();

    let exp = Expression::String(LoopString { value });

    Ok(Node::Expression(exp))
}
