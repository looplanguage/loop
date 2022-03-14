use crate::lexer::token::TokenType;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub body: Block,
    pub else_condition: Box<Option<Node>>,
}

pub fn parse_conditional(p: &mut Parser) -> Option<Node> {
    if !p
        .lexer
        .next_token_is_and_next_token(TokenType::LeftParenthesis)
    {
        p.add_error(format!(
            "Wrong token on line: {}, column: {} -> Got=\"{:?}\". Expected=\"LeftParentheses\"",
            p.lexer.current_line,
            p.lexer.current_col,
            p.lexer.peek_token.clone().unwrap().token
        ));
        return None;
    }

    let condition_node = p.parse_expression(Precedence::Lowest);

    condition_node.as_ref()?;

    if let Node::Expression(exp) = condition_node.unwrap() {
        if !p
            .lexer
            .next_token_and_current_is(TokenType::RightParenthesis)
        {
            p.add_error(format!(
                "wrong token. expected=\"RightParenthesis\". got=\"{:?}\"",
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

        if !p.cur_token_is(TokenType::RightBrace) {
            p.add_error(format!(
                "wrong token. expected=\"RightBrace\". got=\"{:?}\"",
                p.lexer.get_current_token().unwrap().token
            ));
            return None;
        }

        if p.lexer.next_token_is_and_next_token(TokenType::Else) {
            if !p.lexer.next_token_is_and_next_token(TokenType::LeftBrace) {
                p.lexer.next_token();

                return Some(Node::Expression(Expression::Conditional(Box::new(
                    Conditional {
                        condition: Box::new(exp),
                        body,
                        else_condition: Box::new(p.parse_expression(Precedence::Lowest)),
                    },
                ))));
            }

            p.lexer.next_token();

            let else_condition = parse_block(p);

            if !p.cur_token_is(TokenType::RightBrace) {
                p.add_error(format!(
                    "wrong token. expected=\"RightBrace\". got=\"{:?}\"",
                    p.lexer.get_current_token().unwrap().token
                ));
                return None;
            }

            return Some(Node::Expression(Expression::Conditional(Box::new(
                Conditional {
                    condition: Box::new(exp),
                    body,
                    else_condition: Box::new(Some(Node::Statement(Statement::Block(
                        else_condition,
                    )))),
                },
            ))));
        }

        return Some(Node::Expression(Expression::Conditional(Box::new(
            Conditional {
                condition: Box::new(exp),
                body,
                else_condition: Box::new(None),
            },
        ))));
    }

    None
}
