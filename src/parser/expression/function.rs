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
    let mut arguments: Vec<Expression> = parse_expression_arguments(p);

    if !p.cur_token_is(TokenType::RightParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"RightParenthesis\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    };

    // p.lexer.next_token();

    return Some(Node::Expression(Expression::Call(Call {
        identifier: Box::from(left),
        parameters: arguments,
    })));
}

pub fn parse_function(p: &mut Parser) -> Option<Node> {
    println!("STARTING WITH FUNC");
    if !p.lexer.next_is(TokenType::LeftParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"LeftParentheses\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    let mut arguments: Vec<Identifier> = parse_arguments(p);

    p.lexer.next_token();
    p.lexer.next_token();

    println!("STARTING WITH BLOCK");
    let body = parse_block(p);
    println!("DONE WITH BLOCK");

    println!("DONE WITH FUNC");

    Some(Node::Expression(Expression::Function(Function {
        parameters: arguments,
        body,
    })))
}
