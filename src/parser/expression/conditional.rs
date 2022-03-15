use crate::lexer::token::{create_token, TokenType};
use crate::parser::expression::suffix::parse_grouped_expression_without_param;
use crate::parser::expression::Precedence::Lowest;
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

/// Parsing of if-expressions with else blocks
pub fn parse_conditional(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    let uses_parenthesis = p.current_token_is(TokenType::LeftParenthesis);

    // parsing of conditional expression, different types of parsing depending on use of parenthesis
    let condition_node = if uses_parenthesis {
        parse_grouped_expression_without_param(p);
    } else {
        p.parse_expression(Lowest);
    };
    condition_node.as_ref()?;

    p.lexer.next_token();
    if let Node::Expression(exp) = condition_node.unwrap() {
        // Checks if the parenthesis around the if-expression are consistent
        if (p.lexer.current_token.clone().unwrap().token == TokenType::RightParenthesis)
            != uses_parenthesis
        {
            // Custom error whether if-expression has parenthesis or not
            if uses_parenthesis {
                let message = "Syntax  -> for (<condition>) { <code> }\nExample -> if (i < 3) { println(i) }\n\nA loop can be with or without parenthesis".to_string();
                p.throw_exception(
                    create_token(TokenType::RightParenthesis, ")".to_string()),
                    Some(message),
                );
            } else {
                let message = "Syntax  -> for <condition> { <code> }\nExample -> if i < 3 { println(i) }\n\nA loop can be with or without parenthesis".to_string();
                p.throw_exception(
                    create_token(TokenType::LeftBrace, ")".to_string()),
                    Some(message),
                );
            }
        } else if p.lexer.current_token.clone().unwrap().token == TokenType::RightParenthesis {
            // If the if-expression has parenthesis, the lexer needs to go to the next token
            p.lexer.next_token();
        }

        // Parsing the opening brace of the if-expression
        if !p.current_token_is(TokenType::LeftBrace) {
            let message =
                "Syntax  -> for <condition> { <code> }\nExample -> if i < 3 { println(i) }"
                    .to_string();
            p.throw_exception(
                create_token(TokenType::LeftBrace, ")".to_string()),
                Some(message),
            );
        }
        p.lexer.next_token();

        // Parsing the body of the if-expression
        let body = parse_block(p);

        // Parsing the closing of the if-block
        if !p.current_token_is(TokenType::RightBrace) {
            let message =
                "Syntax  -> for <condition> { <code> }\nExample -> if i < 3 { println(i) }"
                    .to_string();
            p.throw_exception(
                create_token(TokenType::RightBrace, "}".to_string()),
                Some(message),
            );
        }

        // Parsing of else block if it exists
        if p.next_token_is(TokenType::Else) {
            p.lexer.next_token();
            return create_conditional(Box::new(exp), body, parse_else(p));
        }

        // Returning if and else body
        return create_conditional(Box::new(exp), body, Box::new(None));
    }

    None
}

/// Parsing of else block of an if-expression
fn parse_else(p: &mut Parser) -> Box<Option<Node>> {
    if !p.lexer.next_token_is_and_next_token(TokenType::LeftBrace) {
        p.lexer.next_token();

        return Box::new(p.parse_expression(Precedence::Lowest));
    }

    p.lexer.next_token();

    let else_condition = parse_block(p);

    if !p.current_token_is(TokenType::RightBrace) {
        let message = "Syntax  -> if <condition> { <code> }\nExample -> if x < 3 { println(x) }\n\nA loop can be with or without parenthesis".to_string();
        p.throw_exception(
            create_token(TokenType::RightBrace, ")".to_string()),
            Some(message),
        );
    }

    Box::new(Some(Node::Statement(Statement::Block(else_condition))))
}

fn create_conditional(
    condition: Box<Expression>,
    body: Block,
    else_condition: Box<Option<Node>>,
) -> Option<Node> {
    Some(Node::Expression(Expression::Conditional(Box::new(
        Conditional {
            condition,
            body,
            else_condition,
        },
    ))))
}
