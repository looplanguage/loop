pub mod expression;
mod program;
pub mod statement;

use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::Statement;
use crate::parser::statement::variable::VariableDeclaration;

use self::statement::variable::parse_variable_declaration;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        let mut current_token: Token = self.lexer.next();

        while current_token.token != TokenType::Eof {
            let new_statement = self.parse_statement(current_token);

            if new_statement.is_some() {
                statements.push(new_statement.unwrap());
            }

            current_token = self.lexer.next();
        }

        Program { statements }
    }

    fn parse_statement(&mut self, token: Token) -> Option<Statement> {
        match token.token {
            TokenType::VariableDeclaration => parse_variable_declaration(self),
            _ => self.parse_expression_statement(token),
        }
    }

    fn parse_expression_statement(&mut self, _token: Token) -> Option<Statement> {
        None
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        Some(Expression::Integer(Integer { value: 0 }))
    }
}

pub fn build_parser(lexer: Lexer) -> Parser {
    Parser { lexer }
}
