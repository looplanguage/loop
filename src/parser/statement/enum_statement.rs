use crate::lexer::token::TokenType;
use crate::parser::expression::{Expression, identifier};
use crate::parser::expression::identifier::Identifier;
use crate::parser::program::Node;
use crate::parser::Parser;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct EnumStatement {
    pub(crate) identifiers: Vec<Identifier>,
}

pub fn parse_enum_statement(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    p.lexer.next_token();

    let mut elements: Vec<Identifier> = Vec::new();

    if !p.lexer.next_current_is(TokenType::LeftBrace) {
        println!("Hello");
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

        if p.lexer.get_current_token().unwrap().token != TokenType::Identifier {
            println!("Hello1");
            p.add_error(format!(
                "wrong token. expected=\"Identifier\". got=\"{:?}\"",
                p.lexer.get_current_token().unwrap().token
            ));
            return None;
        }
        
        elements.push(Identifier {
            value: p.lexer.get_current_token().unwrap().literal.clone(),
        });
        p.lexer.next_token();
    }

    p.lexer.next_token();

    Some(Node::Statement(Statement::EnumStatement(
        EnumStatement { identifiers: elements }),
    ))
}
