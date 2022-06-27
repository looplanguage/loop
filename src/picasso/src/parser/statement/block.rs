use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::expression::hashmap::parse_expression_hashmap;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[allow(dead_code)]
pub fn parse_block_statement(p: &mut Parser) -> Result<Node, SyntaxException> {
    p.lexer.next_token();

    if p.lexer.peek_token.clone().unwrap().token == TokenType::Colon {
        return parse_expression_hashmap(p);
    }

    let block = parse_block(p)?;

    if !p.current_token_is(TokenType::RightBrace) {
        return Err(SyntaxException::ExpectedToken(TokenType::RightBrace));
    }

    Ok(Node::Statement(Statement::Block(Block {
        statements: block.statements,
    })))
}

pub fn parse_block(p: &mut Parser) -> Result<Block, SyntaxException> {
    let mut statements: Vec<Statement> = Vec::new();

    while p.lexer.get_current_token().unwrap().token != TokenType::RightBrace
        && p.lexer.get_current_token().unwrap().token != TokenType::Eof
    {
        let stmt = p.parse_statement(p.lexer.get_current_token().unwrap().clone())?;

        if let Node::Statement(statement) = stmt {
            statements.push(statement)
        } else {
            unreachable!()
        }

        p.lexer.next_token();
    }

    Ok(Block { statements })
}
