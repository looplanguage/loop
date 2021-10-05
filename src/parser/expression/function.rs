use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::statement::block::{parse_block, Block};
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

pub fn parse_function(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::LeftParenthesis) {
        p.add_error(format!(
            "wrong token. got=\"{:?}\". expected=\"LeftParentheses\"",
            p.lexer.current_token.clone().unwrap().token
        ));
        return None;
    }

    let mut arguments: Vec<Identifier> = Vec::new();

    p.lexer.next();

    while p.lexer.current_token.clone().unwrap().token == TokenType::Identifier {
        arguments.push(Identifier {
            value: p.lexer.current_token.clone().unwrap().literal.to_string(),
        });

        p.lexer.next();

        if p.lexer.current_token.clone().unwrap().token == TokenType::Comma {
            p.lexer.next();
        }
    }

    p.lexer.next();
    p.lexer.next();

    let body = parse_block(p);

    Some(Node::Expression(Expression::Function(Function {
        parameters: arguments,
        body,
    })))
}
