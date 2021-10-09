use crate::lexer::token::TokenType;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

pub fn parse_block(p: &mut Parser) -> Block {
    let mut statements: Vec<Statement> = Vec::new();

    while p.lexer.current_token.clone().unwrap().token != TokenType::RightBrace {
        if p.cur_token_is(TokenType::Semicolon) {
            p.lexer.next_token();
            continue;
        }

        let stmt = p.parse_statement(p.lexer.current_token.clone().unwrap());

        if let Some(Node::Statement(statement)) = stmt {
            statements.push(statement)
        } else {
            p.add_error(format!(
                "unable to parse statement at {:?}",
                p.lexer.current_token.clone().unwrap().token
            ))
        }

        if p.peek_token_is(TokenType::RightBrace) {
            p.lexer.next_token();
        }
    }

    Block { statements }
}
