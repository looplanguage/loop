use crate::lexer::token::{Token, TokenType};
use crate::parser::expression::function::{parse_arguments, Parameter};
use crate::parser::expression::Precedence;
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::statement::expression::Expression;
use crate::parser::statement::Statement;
use crate::parser::types::Types;
use crate::parser::Parser;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Method {
    pub name: String,
    pub return_type: Types,
    pub arguments: Vec<Parameter>,
    pub body: Block,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ClassItem {
    Property(Expression),
    Method(Method),
    Lazy(Types),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClassField {
    pub name: String,
    pub index: u32,
    pub item: ClassItem
}

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
    pub values: Vec<ClassField>,
    pub inherits: String,
}

fn parse_class_item(p: &mut Parser, _class_name: String) -> Option<(String, ClassItem)> {
    p.expected(TokenType::Identifier)?;

    if let Some(return_type) = p.parse_type(p.lexer.current_token.as_ref().unwrap().clone()) {
        p.expected(TokenType::Identifier)?;

        let name = p.lexer.current_token.as_ref().unwrap().literal.clone();

        if let Some(_) = p.expected_maybe(TokenType::LeftParenthesis) {
            let parameters = parse_arguments(p);

            p.expected(TokenType::LeftBrace)?;
            p.lexer.next_token();

            let body = parse_block(p);

            Some((
                name.clone(),
                ClassItem::Method(Method {
                    name,
                    return_type,
                    arguments: parameters,
                    body,
                }),
            ))
        } else {
            Some((name.clone(), ClassItem::Lazy(return_type)))
        }
    } else {
        let name = p.lexer.current_token.as_ref().unwrap().literal.clone();

        p.expected(TokenType::Assign)?;
        p.lexer.next_token();

        let mut value = p.parse_expression(Precedence::Lowest)?;
        p.expected_maybe(TokenType::Semicolon);

        if let Node::Expression(exp) = &mut value {
            Some((
                name,
                ClassItem::Property(Expression {
                    expression: Box::new(exp.clone()),
                }),
            ))
        } else {
            None
        }
    }
}

pub fn parse_class_statement(p: &mut Parser) -> Option<Node> {
    p.expected(TokenType::Identifier)?;

    let name = p.lexer.get_current_token().unwrap().literal.clone();
    let mut inherits = String::new();

    if p.expected_maybe(TokenType::LeftArrow).is_some() {
        p.expected(TokenType::Identifier)?;

        inherits = p.lexer.get_current_token().unwrap().literal.clone();
    }

    p.expected(TokenType::LeftBrace);

    let mut values: Vec<ClassField> = Vec::new();

    let mut depth = 1;
    let mut index = 0;
    while depth > 0 {
        let class_item = parse_class_item(p, name.clone())?;

        if let ClassItem::Method(_) = class_item.1 {
            depth += 1;
        }

        values.push(ClassField {
            index,
            name: class_item.0.clone(),
            item: class_item.1
        });
        index += 1;

        while p.next_token_is(TokenType::RightBrace) {
            depth -= 1;

            if depth == 0 {
                break;
            }
        }
    }

    if p.defined_types.contains(&name) {
        p.throw_exception(
            Token {
                token: TokenType::Null,
                literal: "NONE".to_string(),
            },
            Some(format!(
                "Type \"{}\" already defined! (Type definitions are always root scoped)",
                name
            )),
        );
        return None;
    }

    p.defined_types.push(name.clone());

    Some(Node::Statement(Statement::Class(Class {
        inherits,
        name,
        values,
    })))
}
