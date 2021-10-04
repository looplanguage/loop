use crate::parser::expression::Precedence;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub expression: crate::parser::expression::Expression,
}

pub fn parse_expression_statement(p: &mut Parser) -> Option<Statement> {
    let exp = p.parse_expression(Precedence::LOWEST);

    if exp.is_none() {
        return None;
    }

    Some(Statement::Expression(Expression {
        expression: exp.unwrap(),
    }))
}
