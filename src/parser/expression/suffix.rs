use crate::lexer::token::TokenType::RightParenthesis;
use crate::parser::expression::Expression;
use crate::parser::expression::Precedence::Lowest;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Suffix {
    pub(crate) left: Expression,
    pub(crate) operator: String,
    pub(crate) right: Expression,
}

pub fn parse_suffix_expression(p: &mut Parser, left: Expression) -> Option<Node> {
    let operator = p.lexer.current_token.clone().unwrap().literal;

    let pre = p.cur_precedence();

    p.lexer.next_token();

    if let Node::Expression(val) = p.parse_expression(pre).unwrap() {
        return Some(Node::Expression(Expression::Suffix(Box::new(Suffix {
            left,
            operator,
            right: val,
        }))));
    }

    None
}

pub fn parse_grouped_expression(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    let exp = p.parse_expression(Lowest);

    if !p.lexer.next_is(RightParenthesis) {
        p.add_error(format!(
            "wrong token. expected=\")\". got=\"{}\"",
            p.lexer.current_token.clone().unwrap().literal
        ))
    }

    Some(exp.unwrap())
}
