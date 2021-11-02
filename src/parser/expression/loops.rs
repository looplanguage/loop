use crate::lexer::token::TokenType;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::Parser;
use crate::parser::program::Node;
use crate::parser::statement::block::{Block, parse_block};

#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub condition: Box<Expression>,
    pub body: Block,
}

pub fn parse_loop(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::LeftParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"LeftParentheses\"",
            p.lexer.peek_token.clone().unwrap().token
        ));
        return None;
    }

    let condition = p.parse_expression(Precedence::Lowest);

    p.lexer.next_token();

    if !p.lexer.next_current_is(TokenType::LeftBrace) {
        p.add_error(format!(
            "wrong token. expected=\"LeftBrace\". got=\"{:?}\".",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    let body = parse_block(p);

    if !p.cur_token_is(TokenType::RightBrace) {
        p.add_error(format!(
            "wrong token. expected=\"RightBrace\". got=\"{:?}\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    if let Some(Node::Expression(exp)) = condition {
        return Some(Node::Expression(Expression::Loop(Loop {
            condition: Box::from(exp),
            body
        })))
    }

    None
}