use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::{parse_identifier, Identifier};
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub condition: Box<Expression>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopIterator {
    pub identifier: Identifier,
    pub body: Block,
    pub from: u32,
    pub till: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopArrayIterator {
    pub identifier: Identifier,
    pub body: Block,
    pub array: Box<Expression>,
}

pub fn parse_loop(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::LeftParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"LeftParentheses\"",
            p.lexer.peek_token.clone().unwrap().token
        ));
        return None;
    }

    if p.lexer.next_is(TokenType::VariableDeclaration) {
        p.lexer.next_token();

        let identifier = parse_identifier(p);

        if let Some(Node::Expression(Expression::Identifier(ident))) = identifier {
            if !p.lexer.next_is(TokenType::From) {
                if p.lexer.next_is(TokenType::In) {
                    p.lexer.next_token();

                    let exp = p.parse_expression(Precedence::Lowest);

                    if let Some(Node::Expression(expression)) = exp {
                        p.lexer.next_token();
                        p.lexer.next_token();

                        if !p.lexer.next_current_is(TokenType::LeftBrace) {
                            p.add_error(format!(
                                "wrong token. expected=\"LeftBrace\". got=\"{:?}\".",
                                p.lexer.current_token.clone().unwrap().token
                            ));
                            return None;
                        }

                        let body = parse_block(p);

                        return Some(Node::Expression(Expression::LoopArrayIterator(
                            LoopArrayIterator {
                                identifier: ident,
                                body,
                                array: Box::from(expression),
                            },
                        )));
                    }
                } else {
                    p.add_error(format!(
                        "wrong token. got=\"{:?}\". expected=\"From or In\"",
                        p.lexer.peek_token.clone().unwrap().token
                    ));
                    return None;
                }
            }

            if !p.lexer.next_is(TokenType::Integer) {
                p.add_error(format!(
                    "wrong token. got=\"{:?}\". expected=\"Integer\"",
                    p.lexer.peek_token.clone().unwrap().token
                ));
                return None;
            }

            let from = p
                .lexer
                .current_token
                .clone()
                .unwrap()
                .literal
                .parse::<u32>()
                .unwrap();

            if !p.lexer.next_is(TokenType::To) {
                p.add_error(format!(
                    "wrong token. got=\"{:?}\". expected=\"To\"",
                    p.lexer.peek_token.clone().unwrap().token
                ));
                return None;
            }

            if !p.lexer.next_is(TokenType::Integer) {
                p.add_error(format!(
                    "wrong token. got=\"{:?}\". expected=\"Integer\"",
                    p.lexer.peek_token.clone().unwrap().token
                ));
                return None;
            }

            let till = p
                .lexer
                .current_token
                .clone()
                .unwrap()
                .literal
                .parse::<u32>()
                .unwrap();

            p.lexer.next_token();
            p.lexer.next_token();

            if !p.lexer.next_current_is(TokenType::LeftBrace) {
                p.add_error(format!(
                    "wrong token. expected=\"LeftBrace\". got=\"{:?}\".",
                    p.lexer.current_token.clone().unwrap().token
                ));
                return None;
            }

            let body = parse_block(p);

            return Some(Node::Expression(Expression::LoopIterator(LoopIterator {
                identifier: ident,
                body,
                from,
                till,
            })));
        } else {
            p.add_error(format!("expected identifier. got={:?}", identifier));
        }

        return None;
    }

    let condition = p.parse_expression(Precedence::Lowest);

    p.lexer.next_token();

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
            "wrong token. expected=\"RightBrace\". got=\"{:?}\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    if let Some(Node::Expression(exp)) = condition {
        return Some(Node::Expression(Expression::Loop(Loop {
            condition: Box::from(exp),
            body,
        })));
    }

    None
}
