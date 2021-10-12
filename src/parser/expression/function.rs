use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub identifier: Box<Expression>,
    pub parameters: Vec<Expression>,
}

pub fn parse_arguments(p: &mut Parser) -> Vec<Identifier> {
    let mut arguments: Vec<Identifier> = Vec::new();

    p.lexer.next_token();

    while p.lexer.current_token.clone().unwrap().token == TokenType::Identifier {
        arguments.push(Identifier {
            value: p.lexer.current_token.clone().unwrap().literal.to_string(),
        });

        p.lexer.next_token();

        if p.lexer.current_token.clone().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    arguments
}

pub fn parse_expression_arguments(p: &mut Parser) -> Vec<Expression> {
    let mut arguments: Vec<Expression> = Vec::new();

    p.lexer.next_token();

    while p.lexer.current_token.clone().unwrap().token != TokenType::RightParenthesis {
        let exp_node = p.parse_expression(Precedence::Lowest);

        if let Some(Node::Expression(exp)) = exp_node {
            arguments.push(exp);
        }

        p.lexer.next_token();

        if p.lexer.current_token.clone().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    arguments
}

pub fn parse_call(p: &mut Parser, left: Expression) -> Option<Node> {
    let arguments: Vec<Expression> = parse_expression_arguments(p);

    if !p.cur_token_is(TokenType::RightParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"RightParenthesis\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    };

    Some(Node::Expression(Expression::Call(Call {
        identifier: Box::from(left),
        parameters: arguments,
    })))
}

pub fn parse_function(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::LeftParenthesis) {
        p.add_error(format!(
            "wrong token. expected=\"LeftParentheses\". got=\"{:?}\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    let arguments: Vec<Identifier> = parse_arguments(p);

    if !p.lexer.next_current_is(TokenType::RightParenthesis) {
        p.add_error(format!(
            "wrong token. expected=\"RightParenthesis\". got=\"{:?}\".",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

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
            "wrong token. expected=\"RightBrace\". got=\"{:?}\".",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    Some(Node::Expression(Expression::Function(Function {
        parameters: arguments,
        body,
    })))
}
