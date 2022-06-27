use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
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
    pub predefined_type: Option<Types>,
    pub public: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub identifier: Box<Expression>,
    pub parameters: Vec<Expression>,
}

pub fn parse_arguments(p: &mut Parser) -> Result<Vec<Parameter>, SyntaxException> {
    let mut arguments: Vec<Parameter> = Vec::new();

    p.lexer.next_token();

    while p.lexer.get_current_token().unwrap().token == TokenType::Identifier {
        let old = p.lexer.get_current_token().unwrap().clone();

        let tp = {
            if let Some(tpe) = p.parse_type(old.clone()) {
                tpe
            } else {
                return Err(SyntaxException::ExpectedToken(TokenType::RightParenthesis));
            }
        };

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

    Ok(arguments)
}

pub fn parse_expression_arguments(p: &mut Parser) -> Result<Vec<Expression>, SyntaxException> {
    let mut arguments: Vec<Expression> = Vec::new();

    p.lexer.next_token();

    while p.lexer.get_current_token().unwrap().token != TokenType::RightParenthesis
        && p.lexer.get_current_token().unwrap().token != TokenType::Eof
    {
        let exp_node = p.parse_expression(Precedence::Lowest)?;

        if let Node::Expression(exp) = exp_node {
            arguments.push(exp);
        }

        p.lexer.next_token();

        if p.lexer.get_current_token().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }
    }

    Ok(arguments)
}

pub fn parse_call(p: &mut Parser, ident: Expression) -> Result<Node, SyntaxException> {
    let arguments: Vec<Expression> = parse_expression_arguments(p)?;

    p.current_token_is_result(TokenType::RightParenthesis)?;

    Ok(Node::Expression(Expression::Call(Call {
        identifier: Box::from(ident),
        parameters: arguments,
    })))
}

pub fn parse_function(p: &mut Parser) -> Result<Node, SyntaxException> {
    let mut name = String::from("");

    if !p
        .lexer
        .next_token_is_and_next_token(TokenType::LeftParenthesis)
    {
        if p.lexer.next_token_is_and_next_token(TokenType::Identifier) {
            name = p.lexer.current_token.as_ref().unwrap().clone().literal;
            p.lexer.next_token();
        } else {
            return Err(SyntaxException::ExpectedToken(TokenType::LeftParenthesis));
        }
    }

    let arguments: Vec<Parameter> = parse_arguments(p)?;

    p.lexer.next_token();

    if !p.lexer.next_token_and_current_is(TokenType::LeftBrace) {
        return Err(SyntaxException::ExpectedToken(TokenType::LeftBrace));
    }

    let body = parse_block(p)?;

    p.current_token_is_result(TokenType::RightBrace)?;

    let public = {
        let val = p.next_public;

        p.next_public = false;

        val
    };

    Ok(Node::Expression(Expression::Function(Function {
        name,
        parameters: arguments,
        body,
        predefined_type: None,
        public,
    })))
}
