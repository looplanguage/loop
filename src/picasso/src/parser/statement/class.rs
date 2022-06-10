use crate::lexer::token::TokenType;
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
}

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
    pub values: HashMap<String, ClassItem>,
    pub inherits: String,
}

fn parse_class_item(p: &mut Parser, _class_name: String) -> Option<(String, ClassItem)> {
    p.expected(TokenType::Identifier)?;

    if let Some(return_type) = p.parse_type(p.lexer.current_token.as_ref().unwrap().clone()) {
        p.expected(TokenType::Identifier)?;

        let name = p.lexer.current_token.as_ref().unwrap().literal.clone();

        p.expected(TokenType::LeftParenthesis);

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

    let mut values: HashMap<String, ClassItem> = HashMap::new();

    let mut depth = 1;
    while depth > 0 {
        let class_item = parse_class_item(p, name.clone())?;

        if let ClassItem::Method(_) = class_item.1 {
            depth += 1;
        }

        values.insert(class_item.0, class_item.1);

        while p.next_token_is(TokenType::RightBrace) {
            depth -= 1;

            if depth == 0 {
                break;
            }
        }
    }

    //p.lexer.next_token();

    // Box<HashMap<String, (u32, (Types, Expression))>>
    /*
    for value in &mut values {
        if let parser::expression::Expression::Function(f) = &mut *value.1.expression {
            f.parameters.insert(
                0,
                Parameter {
                    identifier: Identifier {
                        value: "self".to_string(),
                    },
                    _type: Types::Basic(BaseTypes::UserDefined(name.clone())),
                },
            );
        }
    }*/

    Some(Node::Statement(Statement::Class(Class {
        inherits,
        name,
        values,
    })))
}
