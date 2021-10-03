pub mod expression;
mod program;
pub mod statement;

use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::Statement;

use self::statement::variable::parse_variable_declaration;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while self.lexer.current_token.clone().unwrap().token != TokenType::Eof {
            let new_statement = self.parse_statement(self.lexer.current_token.clone().unwrap());

            if new_statement.is_some() {
                statements.push(new_statement.unwrap());
            }

            self.lexer.next();
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
