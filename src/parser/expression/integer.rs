use crate::parser::expression::Expression;
use crate::parser::Parser;

#[derive(Debug)]
pub struct Integer {
    pub value: i32,
}

pub fn parse_integer_literal(p: &mut Parser) -> Expression {
    let value = p.lexer.current_token.clone().unwrap().literal.parse::<i32>().unwrap();

    Expression::Integer(Integer { value })
}