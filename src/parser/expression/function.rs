use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::Parser;
use crate::parser::types::{BaseTypes, Types};

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub identifier: Identifier,
    pub _type: Types
}

impl Parameter {
    pub fn get_type(&self) -> String {
        match self._type.clone() {
            Types::Basic(basic) => {
                match basic {
                    BaseTypes::Integer => "int".to_string(),
                    BaseTypes::String => "string".to_string(),
                    BaseTypes::Boolean => "bool".to_string(),
                }
            }
            Types::Array(array) => {
                match array {
                    BaseTypes::Integer => "int[]".to_string(),
                    BaseTypes::String => "string[]".to_string(),
                    BaseTypes::Boolean => "bool[]".to_string(),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
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
        let tp = p.parse_type(p.lexer.get_current_token().unwrap().clone()).unwrap();

        p.lexer.next_token();

        arguments.push(Parameter {
            identifier: Identifier { value: p.lexer.get_current_token().unwrap().literal.to_string() },
            _type: tp
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
    if !p
        .lexer
        .next_token_is_and_next_token(TokenType::LeftParenthesis)
    {
        p.add_error(format!(
            "wrong token. expected=\"LeftParentheses\". got=\"{:?}\"",
            p.lexer.get_current_token().unwrap().token
        ));
        return None;
    }

    let arguments: Vec<Parameter> = parse_arguments(p);

    if !p
        .lexer
        .next_token_and_current_is(TokenType::RightParenthesis)
    {
        p.add_error(format!(
            "wrong token. expected=\"RightParenthesis\". got=\"{:?}\".",
            p.lexer.get_current_token().unwrap().token
        ));
        return None;
    }

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
        parameters: arguments,
        body,
    })))
}
