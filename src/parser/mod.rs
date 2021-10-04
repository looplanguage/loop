pub mod expression;
mod program;
pub mod statement;

use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expression::integer::{Integer, parse_integer_literal};
use crate::parser::expression::{Expression, get_precedence, Precedence};
use crate::parser::program::Program;
use crate::parser::statement::Statement;
use std::collections::HashMap;

use self::statement::variable::parse_variable_declaration;

enum PrefixParser {
    Eof(fn() -> bool),
}

pub struct Parser {
    lexer: Lexer,
    prefix_parser: HashMap<TokenType, fn(parser: &mut Parser) -> Expression>,
    infix_parser: HashMap<TokenType, fn(parser: &mut Parser, expression: Expression) -> Expression>,
    pub errors: Vec<String>
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

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix_parser = self.prefix_parser.get(&self.lexer.current_token.as_ref().unwrap().token);

        if prefix_parser.is_none() {
            self.add_error(format!("no prefix parser for \"{:?}\"", self.lexer.current_token.as_ref().unwrap().token).to_string());
            return None
        }

        let mut expression = prefix_parser.unwrap()(self);


        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix_parser = self.infix_parser.get(&self.lexer.peek_token.as_ref().unwrap().token);

            if infix_parser.is_none() {
                return Some(expression)
            }

            self.lexer.next_token();

            expression = infix_parser.unwrap()(self, expression);
        }

        Some(expression)
    }

    fn add_prefix_parser(&mut self, tok: TokenType, func: fn(parser: &mut Parser) -> Expression) {
        self.prefix_parser.insert(tok, func);
    }

    fn add_infix_parser(&mut self, tok: TokenType, func: fn(parser: &mut Parser, expression: Expression) -> Expression) {
        self.infix_parser.insert(tok, func);
    }

    fn peek_token_is(&self, tok: TokenType) -> bool {
        let peek = self.lexer.peek_token.clone();

        if peek.is_none() {
            return false
        }

        return true
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        get_precedence(self.lexer.peek_token.clone().unwrap().token)
    }

    pub fn cur_precedence(&mut self) -> Precedence {
        get_precedence(self.lexer.current_token.clone().unwrap().token)
    }
}

pub fn build_parser(lexer: Lexer) -> Parser {
    let mut p = Parser {
        lexer,
        prefix_parser: HashMap::new(),
        infix_parser: HashMap::new(),
        errors: Vec::new()
    };

    p.add_prefix_parser(TokenType::Integer, parse_integer_literal);

    return p;
}
