use std::os::macos::raw::stat;
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
        let stmt = p.parse_statement(p.lexer.current_token.clone().unwrap());

        if stmt.is_none() {
            p.add_error(format!("unable to parse statement. see above!"));
        } else {
            if let Node::Statement(statement) = stmt.unwrap() {
                statements.push(statement)
            }
        }

        p.lexer.next();
    }

    Block { statements }
}
