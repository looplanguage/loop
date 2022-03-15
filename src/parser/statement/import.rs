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
    if !p.lexer.next_token_is_and_next_token(TokenType::String) {
        p.add_error(format!(
            "expected string. got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));

        return None;
    }

    let file = p.lexer.get_current_token().unwrap().literal.clone();

    if !p.lexer.next_token_is_and_next_token(TokenType::As) {
        p.add_error(format!(
            "expected keyword \"as\". got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));

        return None;
    }

    if !p.lexer.next_token_is_and_next_token(TokenType::Identifier) {
        p.add_error(format!(
            "expected identifier. got=\"{:?}\"",
            p.lexer.peek_token.clone().unwrap().token
        ));

        return None;
    }

    let identifier = p.lexer.get_current_token().unwrap().literal.clone();

    Some(Node::Statement(Statement::Import(Import {
        file,
        identifier,
    })))
}
