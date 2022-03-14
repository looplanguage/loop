use crate::lexer::token::TokenType;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::string::LoopString;
use crate::parser::expression::Expression;
use crate::parser::expression::Precedence::Lowest;
use crate::parser::program::Node;
use crate::parser::Parser;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum HashableExpression {
    Integer(Integer),
    String(LoopString),
    Boolean(Boolean),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Hashmap {
    pub(crate) values: HashMap<HashableExpression, Expression>,
}

pub fn parse_expression_hashmap(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let mut values: HashMap<HashableExpression, Expression> = HashMap::new();

    while p.lexer.get_current_token().unwrap().token != TokenType::RightBrace
        && p.lexer.get_current_token().unwrap().token != TokenType::Eof
    {
        if p.lexer.get_current_token().unwrap().token == TokenType::Comma {
            p.lexer.next_token();
        }

        let key_exp = p.parse_expression(Lowest);

        if let Some(Node::Expression(key)) = key_exp {
            if !p.lexer.next_token_is_and_next_token(TokenType::Colon) {
                p.add_error(format!(
                    "wrong token. expected=\"Colon\". got=\"{:?}\"",
                    p.lexer.peek_token.clone().unwrap().token
                ));
                return None;
            }

            p.lexer.next_token();
            let val_exp = p.parse_expression(Lowest);

            if let Some(Node::Expression(val)) = val_exp {
                let hashable = key.get_hash();

                if let Some(hash) = hashable {
                    values.insert(hash, val);
                } else {
                    p.add_error(format!(
                        "type is not hashable therefore can not be used as key. got=\"{:?}\"",
                        key
                    ));
                    return None;
                }
            }
        }

        p.lexer.next_token();
    }

    if p.lexer.get_current_token().unwrap().token != TokenType::RightBrace {
        p.add_error(format!(
            "wrong token. expected=\"RightBrace\". got=\"{:?}\"",
            p.lexer.get_current_token().unwrap().token
        ));

        return None;
    }

    Some(Node::Expression(Expression::Hashmap(Hashmap { values })))
}
