use crate::lexer::token::{create_token, TokenType};
use crate::parser::exception::SyntaxException;
use crate::parser::expression::identifier::{parse_identifier, Identifier};
use crate::parser::expression::Expression;
use crate::parser::expression::Precedence::Lowest;
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

pub fn parse_loop(p: &mut Parser) -> Result<Node, SyntaxException> {
    p.lexer.next_token();
    let uses_parenthesis = p.current_token_is(TokenType::LeftParenthesis);
    if uses_parenthesis {
        p.lexer.next_token();
    }
    if p.current_token_is(TokenType::VariableDeclaration) {
        p.lexer.next_token();

        let identifier = parse_identifier(p)?;

        return if let Node::Expression(Expression::Identifier(ident)) = identifier {
            if !p.lexer.next_token_is_and_next_token(TokenType::Assign) {
                parse_loop_array_iterator(p, ident, uses_parenthesis)
            } else {
                parse_loop_iterator(p, ident, uses_parenthesis)
            }
        } else {
            Err(SyntaxException::ExpectedToken(TokenType::Identifier))
        };
    }

    // Regular for loop: for i < 3 { STATEMENTS }
    let condition_node = p.parse_expression(Lowest)?;

    p.lexer.next_token();
    // Checks if the parenthesis around the if-expression are consistent
    if (p.current_token_is(TokenType::RightParenthesis)) != uses_parenthesis {
        // Custom error whether if-expression has parenthesis or not
        if uses_parenthesis {
            let message = "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }\n\nAn if expression can be with or without parenthesis".to_string();
            return Err(SyntaxException::CustomMessage(
                "expected: RightParenthesis".to_string(),
                Some(message),
            ));
        } else {
            let message = "Syntax  -> for <condition> { <code> }\nExample -> for i < 3 { println(i) }\n\nAn if expression can be with or without parenthesis".to_string();
            return Err(SyntaxException::CustomMessage(
                "expected: NoParenthesis".to_string(),
                Some(message),
            ));
        }
    } else if uses_parenthesis {
        // If the if-expression has parenthesis, the lexer needs to go to the next token
        p.lexer.next_token();
    }

    if !p.current_token_is(TokenType::LeftBrace) {
        let message =
            "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }"
                .to_string();
        return Err(SyntaxException::CustomMessage(
            "expected: LeftBrace".to_string(),
            Some(message),
        ));
    }
    p.lexer.next_token();

    let body = parse_block(p)?;

    if !p.current_token_is(TokenType::RightBrace) {
        let message =
            "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }"
                .to_string();
        return Err(SyntaxException::CustomMessage(
            "expected: RightBrace".to_string(),
            Some(message),
        ));
    }

    if let Node::Expression(exp) = condition_node {
        return Ok(Node::Expression(Expression::Loop(Loop {
            condition: Box::from(exp),
            body,
        })));
    }

    Err(SyntaxException::Unknown)
}

/// Parsing the array iterator loop:
///
/// Loop iterator:
/// ```loop
/// var array = [1, 2, 3]
/// for var i in array {
///     println(i)
/// }
/// ```
pub fn parse_loop_array_iterator(
    p: &mut Parser,
    identifier: Identifier,
    uses_parenthesis: bool,
) -> Result<Node, SyntaxException> {
    p.lexer.next_token();
    if p.current_token_is(TokenType::In) {
        p.lexer.next_token();

        let exp = p.parse_expression(Lowest)?;

        if let Node::Expression(expression) = exp {
            p.lexer.next_token();
            if uses_parenthesis {
                p.lexer.next_token();
            }

            p.current_token_is_result(TokenType::LeftBrace)?;
            p.lexer.next_token();

            let body = parse_block(p)?;

            return Ok(Node::Expression(Expression::LoopArrayIterator(
                LoopArrayIterator {
                    identifier,
                    body,
                    array: Box::from(expression),
                },
            )));
        }
    } else {
        return Err(SyntaxException::ExpectedToken(TokenType::From));
    }

    Err(SyntaxException::Unknown)
}

/// Parsing the iterator loop:
///
/// Loop iterator:
/// ```loop
/// for var i = 0 to 100 {
///     println(i)
/// }
/// ```
pub fn parse_loop_iterator(
    p: &mut Parser,
    identifier: Identifier,
    uses_parenthesis: bool,
) -> Result<Node, SyntaxException> {
    p.lexer.next_token();
    if !p.current_token_is(TokenType::Integer) {
        let message =
            "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }"
                .to_string();
        p.throw_exception(
            create_token(TokenType::Integer, "Integer".to_string()),
            Some(message),
        );
    }

    let from = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<u32>()
        .unwrap();

    p.lexer.next_token();
    if !p.current_token_is(TokenType::To) {
        let message =
            "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }"
                .to_string();
        p.throw_exception(create_token(TokenType::To, "to".to_string()), Some(message));
    }

    p.lexer.next_token();
    if !p.current_token_is(TokenType::Integer) {
        let message =
            "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }"
                .to_string();
        p.throw_exception(
            create_token(TokenType::Integer, "Integer".to_string()),
            Some(message),
        );
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

    // Checks if the parenthesis around the if-expression are consistent
    if (p.current_token_is(TokenType::RightParenthesis)) != uses_parenthesis {
        // Custom error whether if-expression has parenthesis or not
        if uses_parenthesis {
            let message = "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }\n\nAn if expression can be with or without parenthesis".to_string();
            p.throw_exception(
                create_token(TokenType::RightParenthesis, ")".to_string()),
                Some(message),
            );
        } else {
            let message = "Syntax  -> for <condition> { <code> }\nExample -> for i < 3 { println(i) }\n\nAn if expression can be with or without parenthesis".to_string();
            p.throw_exception(
                create_token(TokenType::LeftBrace, ")".to_string()),
                Some(message),
            );
        }
    } else if uses_parenthesis {
        // If the if-expression has parenthesis, the lexer needs to go to the next token
        p.lexer.next_token();
    }

    if !p.current_token_is(TokenType::LeftBrace) {
        let message =
            "Syntax  -> for (<condition>) { <code> }\nExample -> for (i < 3) { println(i) }"
                .to_string();
        p.throw_exception(
            create_token(TokenType::LeftBrace, "{".to_string()),
            Some(message),
        );
    }
    p.lexer.next_token();
    let body = parse_block(p)?;

    Ok(Node::Expression(Expression::LoopIterator(LoopIterator {
        identifier,
        body,
        from,
        till,
    })))
}
