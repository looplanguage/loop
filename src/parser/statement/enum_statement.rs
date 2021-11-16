use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::program::Node;
use crate::parser::Parser;
use std::collections::HashMap;

pub struct EnumStatement {
    pub(crate) values: HashMap<Identifier, i32>,
}

pub fn parse_enum_statement(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    p.lexer.next_token();

    if !p.lexer.next_current_is(TokenType::LeftBrace) {
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
    }
    return None;
}
