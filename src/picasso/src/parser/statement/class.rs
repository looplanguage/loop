use crate::lexer::token::TokenType;
use crate::parser::expression::Precedence;
use crate::parser::program::Node;
use crate::parser::statement::expression::Expression;
use crate::parser::statement::Statement;
use crate::parser::types::Types;
use crate::parser::Parser;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    pub name: String,
    pub values: HashMap<String, Expression>,
}

fn parse_class_item(p: &mut Parser) -> Option<(String, Expression)> {
    p.expected(TokenType::Identifier)?;

    let name = p.lexer.current_token.as_ref().unwrap().literal.clone();

    p.expected(TokenType::Assign)?;
    p.lexer.next_token();

    let value = p.parse_expression(Precedence::Lowest)?;
    p.expected_maybe(TokenType::Semicolon);

    if let Node::Expression(exp) = value {
        Some((
            name,
            Expression {
                expression: Box::new(exp),
            },
        ))
    } else {
        None
    }
}

pub fn parse_class_statement(p: &mut Parser) -> Option<Node> {
    p.expected(TokenType::Identifier)?;

    let name = p.lexer.get_current_token().unwrap().literal.clone();

    p.expected(TokenType::LeftBrace);

    let mut values: HashMap<String, Expression> = HashMap::new();

    while !p.current_token_is(TokenType::RightBrace) {
        let class_item = parse_class_item(p)?;
        values.insert(class_item.0, class_item.1);

        if p.next_token_is(TokenType::RightBrace) {
            break;
        }
    }

    p.lexer.next_token();

    Some(Node::Statement(Statement::Class(Class { name, values })))
}
