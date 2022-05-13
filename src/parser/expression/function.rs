use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::types::Types;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub identifier: Identifier,
    pub _type: Types,
}

impl Parameter {
    pub fn get_type(&self) -> String {
        self._type.transpile()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub identifier: Box<Expression>,
    pub parameters: Vec<Expression>,
}

pub fn parse_arguments(p: &mut Parser) -> Vec<Parameter> {
    let mut arguments: Vec<Parameter> = Vec::new();

    p.lexer.next_token();

    while p.lexer.get_current_token().unwrap().token == TokenType::Identifier {
        let old = p.lexer.get_current_token().unwrap().clone();
        let tp = p.parse_type(old.clone()).unwrap();

        p.lexer.next_token_is_and_next_token(TokenType::Identifier);
        arguments.push(Parameter {
            identifier: Identifier {
                value: p.lexer.get_current_token().unwrap().literal.to_string(),
            },
            _type: tp,
        });

        p.lexer.next_token();

        if p.lexer.get_current_token().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    arguments
}

pub fn parse_expression_arguments(p: &mut Parser) -> Vec<Expression> {
    let mut arguments: Vec<Expression> = Vec::new();

    p.lexer.next_token();

    while p.lexer.get_current_token().unwrap().token != TokenType::RightParenthesis
        && p.lexer.get_current_token().unwrap().token != TokenType::Eof
    {
        let exp_node = p.parse_expression(Precedence::Lowest);

        if let Some(Node::Expression(exp)) = exp_node {
            arguments.push(exp);
        }

        p.lexer.next_token();

        if p.lexer.get_current_token().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    arguments
}

pub fn parse_call(p: &mut Parser, left: Expression) -> Option<Node> {
    let arguments: Vec<Expression> = parse_expression_arguments(p);

    if !p.current_token_is(TokenType::RightParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"RightParenthesis\"",
            p.lexer.get_current_token().unwrap().token
        ));
        return None;
    };

    Some(Node::Expression(Expression::Call(Call {
        identifier: Box::from(left),
        parameters: arguments,
    })))
}

pub fn parse_function(p: &mut Parser) -> Option<Node> {
    let mut name = String::from("");

    if !p
        .lexer
        .next_token_is_and_next_token(TokenType::LeftParenthesis)
    {
        if p.lexer.next_token_is_and_next_token(TokenType::Identifier) {
            name = p.lexer.current_token.as_ref().unwrap().clone().literal;
            p.lexer.next_token();
        } else {
            p.add_error(format!(
                "wrong token. expected=\"LeftParentheses\". got=\"{:?}\"",
                p.lexer.get_current_token().unwrap().token
            ));
            return None;
        }
    }

    let arguments: Vec<Parameter> = parse_arguments(p);

    p.lexer.next_token();

    if !p.lexer.next_token_and_current_is(TokenType::LeftBrace) {
        p.add_error(format!(
            "wrong token. expected=\"LeftBrace\". got=\"{:?}\".",
            p.lexer.get_current_token().unwrap().token
        ));
        return None;
    }

    let body = parse_block(p);

    if !p.current_token_is(TokenType::RightBrace) {
        p.add_error(format!(
            "wrong token. expected=\"RightBrace\". got=\"{:?}\".",
            p.lexer.get_current_token().unwrap().token
        ));
        return None;
    }

    Some(Node::Expression(Expression::Function(Function {
        name,
        parameters: arguments,
        body,
    })))
}
