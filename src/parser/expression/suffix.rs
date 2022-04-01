use crate::lexer::token::TokenType;
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

impl Suffix {
    pub fn transpile(&self) -> Option<String> {
        Some(format!("{} {} {}", self.left.clone().get_value().unwrap(), self.operator, self.right.clone().get_value().unwrap()))
    }
}

pub fn parse_suffix_expression(p: &mut Parser, left: Expression) -> Option<Node> {
    let operator = p.lexer.get_current_token().unwrap().literal.clone();

    let pre = p.current_precedence();

    p.lexer.next_token();

    let exp = p.parse_expression(pre);

    exp.as_ref()?;

    if let Node::Expression(val) = exp.unwrap() {
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

    if exp.is_none() {
        p.add_error("unable to parse group. expected=\"Expression\" got=\"null\"".to_string());
        return None;
    }

    p.lexer.next_token();
    if !p.current_token_is(TokenType::RightParenthesis) {
        p.add_error(format!(
            "wrong token. expected=\"RightParenthesis\". got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));
        return None;
    }

    Some(exp.unwrap())
}

/// Same as: `parse_grouped_expression`, except it is for for and if expressions, without any parenthesis
///
/// Returns: `Option<Node>`
pub fn parse_grouped_expression_without_param(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    let exp = p.parse_expression(Lowest);

    if exp.is_none() {
        p.add_error("unable to parse group. expected=\"Expression\" got=\"null\"".to_string());
        return None;
    }

    Some(exp.unwrap())
}
