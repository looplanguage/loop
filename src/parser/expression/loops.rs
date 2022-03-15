use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::{parse_identifier, Identifier};
use crate::parser::expression::Precedence::Lowest;
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

// TODO: Stack overflow with above 2048 loops, probably not popping enough
pub fn parse_loop(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    let uses_parenthesis = p.current_token_is(TokenType::LeftParenthesis);
    if uses_parenthesis {
        p.lexer.next_token();
    }
    if p.current_token_is(TokenType::VariableDeclaration) {
        p.lexer.next_token();

        let identifier = parse_identifier(p);

        if let Some(Node::Expression(Expression::Identifier(ident))) = identifier {
            if !p.lexer.next_token_is_and_next_token(TokenType::Assign) {
                if p.lexer.next_token_is_and_next_token(TokenType::In) {
                    p.lexer.next_token();

                    let exp = p.parse_expression(Precedence::Lowest);

                    if let Some(Node::Expression(expression)) = exp {
                        p.lexer.next_token();
                        if uses_parenthesis {
                            p.lexer.next_token();
                        }

                        if !p.lexer.next_token_and_current_is(TokenType::LeftBrace) {
                            p.lexer.current_token.clone().unwrap().display();

                            p.add_error(format!(
                                "wrong token. expected=\"LeftBrace\". got=\"{:?}\".",
                                p.lexer.get_current_token().unwrap().token
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

            if !p.lexer.next_token_is_and_next_token(TokenType::Integer) {
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

            if !p.lexer.next_token_is_and_next_token(TokenType::To) {
                p.add_error(format!(
                    "wrong token. got=\"{:?}\". expected=\"To\"",
                    p.lexer.peek_token.clone().unwrap().token
                ));
                return None;
            }

            if !p.lexer.next_token_is_and_next_token(TokenType::Integer) {
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

            if uses_parenthesis {
                p.lexer.next_token();
            }
            p.lexer.next_token();
            if !p.lexer.next_token_and_current_is(TokenType::LeftBrace) {
                p.add_error(format!(
                    "wrong token. expected=\"LeftBrace\". got=\"{:?}\".",
                    p.lexer.get_current_token().unwrap().token
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

    let condition_node = p.parse_expression(Lowest);
    condition_node.as_ref()?;

    p.lexer.next_token();
    // ToDo: Give error when it begins with parenthesis but does not end with it.
    if uses_parenthesis {
        p.lexer.next_token();
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
            "wrong token. expected=\"RightBrace\". got=\"{:?}\"",
            p.lexer.get_current_token().unwrap().token
        ));
        return None;
    }

    if let Some(Node::Expression(exp)) = condition_node {
        return Some(Node::Expression(Expression::Loop(Loop {
            condition: Box::from(exp),
            body,
        })));
    }

    None
}
