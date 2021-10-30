use crate::lexer::token::TokenType;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Clone, PartialEq, Debug)]
pub struct Import {
    pub file: String,
    pub identifier: String,
}

pub fn parse_import_statement(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::String) {
        p.add_error(format!(
            "expected string. got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));

        return None;
    }

    let file = p.lexer.current_token.clone().unwrap().literal;

    if !p.lexer.next_is(TokenType::As) {
        p.add_error(format!(
            "expected keyword \"as\". got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));

        return None;
    }

    if !p.lexer.next_is(TokenType::Identifier) {
        p.add_error(format!(
            "expected identifier. got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));

        return None;
    }

    let identifier = p.lexer.current_token.clone().unwrap().literal;

    Some(Node::Statement(Statement::Import(Import {
        file,
        identifier,
    })))
}
