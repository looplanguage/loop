use crate::parser::expression::Expression;
use crate::parser::Parser;

#[derive(Debug)]
pub struct Suffix {
    left: Expression,
    operator: char,
    right: Expression
}

pub fn parse_suffix_expression(p: &mut Parser, expression: Expression) -> Expression {
    let operator = p.lexer.current_token.clone().unwrap().literal.chars().nth(0).unwrap();

    let pre = p.cur_precedence();

    p.lexer.next();

    Expression::Suffix(Box::new(Suffix {
        left: expression,
        operator,
        right: p.parse_expression(pre).unwrap()
    }))
}