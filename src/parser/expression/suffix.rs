use crate::lexer::token::TokenType::RightParenthesis;
use crate::parser::expression::Expression;
use crate::parser::expression::Precedence::LOWEST;
use crate::parser::Parser;

#[derive(Debug, PartialEq)]
pub struct Suffix {
    pub(crate) left: Expression,
    pub(crate) operator: char,
    pub(crate) right: Expression,
}

pub fn parse_suffix_expression(p: &mut Parser, expression: Expression) -> Expression {
    let operator = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .chars()
        .nth(0)
        .unwrap();

    let pre = p.cur_precedence();

    p.lexer.next();

    Expression::Suffix(Box::new(Suffix {
        left: expression,
        operator,
        right: p.parse_expression(pre).unwrap(),
    }))
}

pub fn parse_grouped_expression(p: &mut Parser) -> Expression {
    p.lexer.next();
    let exp = p.parse_expression(LOWEST);

    if !p.lexer.next_is(RightParenthesis) {
        p.add_error(format!(
            "wrong token. expected=\")\". got=\"{}\"",
            p.lexer.current_token.clone().unwrap().literal
        ))
    }

    return exp.unwrap();
}
