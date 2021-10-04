use crate::parser::expression::Expression;
use crate::parser::Parser;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub(crate) value: String,
}

pub fn parse_identifier(p: &mut Parser) -> Expression {
    Expression::Identifier(Identifier {
        value: p.lexer.current_token.clone().unwrap().literal,
    })
}
