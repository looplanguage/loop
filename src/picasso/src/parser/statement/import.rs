use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Import {
    pub file: String,
    pub identifier: String,
}

pub fn parse_import_statement(p: &mut Parser) -> Result<Node, SyntaxException> {
    p.lexer
        .next_token_is_and_next_token_result(TokenType::String)?;

    let file = p.lexer.get_current_token().unwrap().literal.clone();

    p.lexer.next_token_is_and_next_token_result(TokenType::As)?;

    p.lexer
        .next_token_is_and_next_token_result(TokenType::Identifier)?;

    let identifier = p.lexer.get_current_token().unwrap().literal.clone();

    Ok(Node::Statement(Statement::Import(Import {
        file,
        identifier,
    })))
}
