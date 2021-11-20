use crate::lexer::token::{Token, TokenType};
use crate::parser::expression::identifier::{parse_identifier, Identifier};
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use crate::parser::program::Node;
use crate::parser::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum HashableExpressionEnum {
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Enum {
    pub(crate) values: HashMap<HashableExpressionEnum, i32>,
}

pub fn parse_expression_enum(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    p.lexer.next_token();

    let mut counter: i32 = 0;
    let mut values: HashMap<HashableExpressionEnum, i32> = HashMap::new();

    if !p.lexer.next_current_is(TokenType::LeftBrace) {
        p.add_error(format!(
            "wrong token. expected=\"LeftBrace\". got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));
        return None;
    }

    while p.lexer.get_current_token().unwrap().token != TokenType::RightBrace
        && p.lexer.get_current_token().unwrap().token != TokenType::Eof
    {
        if p.lexer.get_current_token().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }

        let y = parse_identifier(p);

        if let Some(Node::Expression(constant)) = y {
            if !p.lexer.next_is(TokenType::Identifier) {
                p.add_error(format!(
                    "wrong token. expected=\"Identifier\". got=\"{:?}\"",
                    p.lexer.peek_token.clone().unwrap().token
                ));
                return None;
            }
        }
        counter += 1;
    }

    Some(Node::Expression(Expression::Enum(EnumExpression { values })))
}
