use crate::lexer::token::TokenType;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Clone, PartialEq, Debug)]
pub struct Export {
    pub names: Vec<String>,
}

pub fn parse_export_statement(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let mut names = Vec::new();
    while p.lexer.get_current_token().unwrap().token != TokenType::RightBrace {
        p.lexer.next_token();

        let name = p.lexer.current_token.clone().unwrap().literal;
        names.push(name);

        p.lexer.next_token();
    }

    Some(Node::Statement(Statement::Export(Export { names })))
}
