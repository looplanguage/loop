use crate::lexer::token::TokenType;
use crate::parser::Parser;
use crate::parser::program::Node;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

pub fn parse_block(p: &mut Parser) -> Block {
    let mut statements: Vec<Statement> = Vec::new();

    while p.lexer.current_token.clone().unwrap().token != TokenType::RightBrace {
        let stmt = p.parse_statement(p.lexer.current_token.clone().unwrap());

        if stmt.is_some() {
            if let Node::Statement(statement) = stmt.unwrap() {
                statements.push(statement)
            }
        }

        p.lexer.next();
    }

    Block {
        statements
    }
}