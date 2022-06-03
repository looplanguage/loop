//! Responsible for parsing tokens into an abstract syntax tree
use std::collections::HashMap;

use crate::exception::Exception;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::parser::expression::array::parse_expression_array;
use crate::parser::expression::boolean::{parse_boolean, parse_inverted_boolean};
use crate::parser::expression::conditional::parse_conditional;
use crate::parser::expression::function::{parse_call, parse_function};
use crate::parser::expression::hashmap::parse_expression_hashmap;
use crate::parser::expression::identifier::parse_identifier;
use crate::parser::expression::index::parse_index_expression;
use crate::parser::expression::loops::parse_loop;
use crate::parser::expression::null::parse_expression_null;
use crate::parser::expression::string::parse_string_literal;
use crate::parser::expression::suffix::{parse_grouped_expression, parse_suffix_expression};
use crate::parser::expression::{get_precedence, Expression, Precedence};
use crate::parser::program::{Node, Program};
use crate::parser::statement::assign::parse_variable_assignment;
use crate::parser::statement::constant::parse_constant_declaration;
use crate::parser::statement::expression::parse_expression_statement;
use crate::parser::statement::return_statement::parse_return_statement;
use crate::parser::statement::Statement;

use self::statement::variable::parse_variable_declaration;
use crate::exception::syntax::{throw_syntax_error, SyntaxError};
use crate::parser::expression::number::{parse_negative_number, parse_number_literal};
use crate::parser::statement::break_statement::parse_break_statement;
use crate::parser::statement::class::parse_class_statement;
use crate::parser::statement::export::parse_export_statement;
use crate::parser::statement::import::parse_import_statement;
use crate::parser::types::{BaseTypes, FunctionType, Types};

pub mod expression;
pub mod program;
pub mod statement;
mod test;
pub mod types;

type PrefixParseFn = fn(parser: &mut Parser) -> Option<Node>;
type InfixParseFn = fn(parser: &mut Parser, expression: Expression) -> Option<Node>;

// The parser itself, containing metadata needed during the parsing process
pub struct Parser {
    lexer: Lexer,
    prefix_parser: HashMap<TokenType, PrefixParseFn>,
    infix_parser: HashMap<TokenType, InfixParseFn>,
    pub errors: Vec<Exception>,
}

impl Parser {
    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while self.lexer.get_current_token().unwrap().token != TokenType::Eof {
            let tok = self.lexer.get_current_token().unwrap().clone();
            let new_statement = self.parse_statement(tok);

            if let Some(Node::Statement(i)) = new_statement {
                statements.push(i);
            }

            self.lexer.next_token();
        }

        Program { statements }
    }

    fn expected(&mut self, token: TokenType) -> Option<()> {
        if !self.lexer.next_token_is_and_next_token(token) {
            self.throw_exception(
                Token {
                    token,
                    literal: "".to_string(),
                },
                None,
            );

            return None;
        }

        Some(())
    }

    fn expected_maybe(&mut self, token: TokenType) -> Option<()> {
        if !self.lexer.next_token_is_and_next_token(token) {
            return None;
        }

        Some(())
    }

    fn peek_is_array(&mut self) -> bool {
        if self.peek_token_is(TokenType::LeftBracket) {
            self.lexer.next_token();

            if self.peek_token_is(TokenType::RightBracket) {
                self.lexer.next_token();

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn parse_type(&mut self, token: Token) -> Option<Types> {
        match token.token {
            TokenType::Identifier => match token.literal.as_str() {
                "int" => {
                    if self.peek_is_array() {
                        Some(Types::Array(Box::from(Types::Basic(BaseTypes::Integer))))
                    } else {
                        Some(Types::Basic(BaseTypes::Integer))
                    }
                }
                "bool" => {
                    if self.peek_is_array() {
                        Some(Types::Array(Box::from(Types::Basic(BaseTypes::Boolean))))
                    } else {
                        Some(Types::Basic(BaseTypes::Boolean))
                    }
                }
                "string" => {
                    if self.peek_is_array() {
                        Some(Types::Array(Box::from(Types::Basic(BaseTypes::String))))
                    } else {
                        Some(Types::Basic(BaseTypes::String))
                    }
                }
                "float" => {
                    if self.peek_is_array() {
                        Some(Types::Array(Box::from(Types::Basic(BaseTypes::Float))))
                    } else {
                        Some(Types::Basic(BaseTypes::Float))
                    }
                }
                "void" => Some(Types::Void),
                // Function types are as follows: func<arg1,arg2,arg3><retType>
                "func" => {
                    let mut func_type = FunctionType {
                        return_type: Box::new(Types::Void),
                        parameter_types: vec![],
                        reference: "".to_string(),
                    };

                    // <
                    self.lexer.next_token();
                    self.lexer.next_token();
                    let mut skipped = false;

                    // func<INT
                    while !self.current_token_is(TokenType::RightArrow)
                        && !self.current_token_is(TokenType::Eof)
                    {
                        let next = self.lexer.current_token.as_ref().unwrap().clone();
                        let tp = self.parse_type(next);

                        func_type.parameter_types.push(tp.unwrap());

                        // Comma
                        skipped = true;
                        self.lexer.next_token();
                        if self.lexer.current_token.as_ref().unwrap().token == TokenType::Comma {
                            self.lexer.next_token();
                        }
                    }

                    // '>'
                    self.lexer.next_token();
                    // Return type '<'
                    if !skipped {
                        self.lexer.next_token();
                    }

                    self.lexer.next_token();
                    let cur = self.lexer.current_token.as_ref().unwrap().clone();

                    func_type.return_type = Box::new(self.parse_type(cur).unwrap());

                    // previous type & '>'
                    self.lexer.next_token();
                    self.lexer.next_token();

                    Some(Types::Function(func_type))
                }
                _ => {
                    if self.peek_is_array() {
                        Some(Types::Array(Box::from(Types::Basic(
                            BaseTypes::UserDefined(token.literal),
                        ))))
                    } else {
                        //Some(Types::Basic(BaseTypes::UserDefined(token.literal)))
                        None
                    }
                }
            },
            TokenType::VariableDeclaration => Some(Types::Auto),
            _ => None,
        }
    }

    fn parse_statement(&mut self, token: Token) -> Option<Node> {
        let r = match token.token {
            TokenType::ConstantDeclaration => parse_constant_declaration(self),
            TokenType::Identifier => {
                if self.peek_token_is(TokenType::Assign) {
                    parse_variable_assignment(self)
                } else if self.peek_token_is(TokenType::Colon) {
                    parse_variable_declaration(self, None)
                } else if self.peek_token_is(TokenType::Identifier) {
                    // User has explicitly typed a variable.
                    let types = self
                        .parse_type(self.lexer.get_current_token().unwrap().clone())
                        .unwrap();
                    self.lexer.next_token();
                    parse_variable_declaration(self, Some(types))
                } else if self.peek_token_is(TokenType::LeftArrow) {
                    let types = self
                        .parse_type(self.lexer.get_current_token().unwrap().clone())
                        .unwrap();
                    parse_variable_declaration(self, Some(types))
                } else {
                    parse_expression_statement(self)
                }
            }
            TokenType::Return => parse_return_statement(self),
            //TokenType::LeftBrace => parse_block_statement(self),
            TokenType::Import => parse_import_statement(self),
            TokenType::Export => parse_export_statement(self),
            TokenType::Break => parse_break_statement(self),
            TokenType::Class => parse_class_statement(self),
            _ => self.parse_expression_statement(),
        };

        if self.lexer.peek_token.is_some()
            && self.lexer.peek_token.as_ref().unwrap().token == TokenType::Semicolon
        {
            self.lexer.next_token();
        }

        r
    }

    fn parse_expression_statement(&mut self) -> Option<Node> {
        parse_expression_statement(self)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Node> {
        let prefix_parser = self
            .prefix_parser
            .get(&self.lexer.current_token.as_ref().unwrap().token);

        if prefix_parser.is_none() {
            self.add_error(format!(
                "no prefix parser for \"{:?}\"",
                self.lexer.current_token.as_ref().unwrap().token
            ));

            return None;
        }

        let expression_node: Option<Node> = prefix_parser.unwrap()(self);
        expression_node.as_ref()?;

        if let Node::Expression(exp) = expression_node.unwrap() {
            let mut infix_expression_node: Option<Node> = None;
            while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
                let infix_parser = self
                    .infix_parser
                    .get(&self.lexer.peek_token.as_ref().unwrap().token);

                if infix_parser.is_none() {
                    return Some(Node::Expression(exp));
                }

                self.lexer.next_token();

                if infix_expression_node.is_some() {
                    if let Node::Expression(a) = infix_expression_node.clone().unwrap() {
                        // Calling parser functions from the hashmap
                        infix_expression_node = infix_parser.unwrap()(self, a);
                    }
                } else {
                    // Calling parser functions from the hashmap
                    infix_expression_node = infix_parser.unwrap()(self, exp.clone())
                }
            }

            if infix_expression_node.is_some() {
                return infix_expression_node;
            }

            return Some(Node::Expression(exp));
        }

        throw_syntax_error(
            self.lexer.current_line - self.lexer.current_token.clone().unwrap().literal_len(),
            self.lexer.current_col,
            self.lexer.get_line(self.lexer.current_line),
            self.lexer.current_token.clone().unwrap().literal,
        );

        None
    }

    fn add_prefix_parser(&mut self, tok: TokenType, func: fn(parser: &mut Parser) -> Option<Node>) {
        self.prefix_parser.insert(tok, func);
    }

    fn add_infix_parser(
        &mut self,
        tok: TokenType,
        func: fn(parser: &mut Parser, expression: Expression) -> Option<Node>,
    ) {
        self.infix_parser.insert(tok, func);
    }

    fn peek_token_is(&self, tok: TokenType) -> bool {
        let peek = self.lexer.peek_token.clone();

        if peek.is_none() {
            return false;
        }

        peek.unwrap().token == tok
    }

    pub fn current_token_is(&self, tok: TokenType) -> bool {
        let cur = self.lexer.get_current_token();

        if cur.is_none() {
            return false;
        }

        cur.unwrap().token == tok
    }

    pub fn next_token_is(&self, tok: TokenType) -> bool {
        let cur = self.lexer.get_peek_token();

        if cur.is_none() {
            return false;
        }

        cur.unwrap().token == tok
    }

    pub fn add_error(&mut self, error: String) {
        /*
        sentry::with_scope(
            |scope| {
                scope.set_tag("exception.type", "parser");
            },
            || {
                sentry::capture_message(error.as_str(), sentry::Level::Info);
            },
        );*/

        self.errors.push(Exception::Syntax(error));
    }

    pub fn peek_precedence(&mut self) -> Precedence {
        get_precedence(self.lexer.peek_token.clone().unwrap().token)
    }

    pub fn current_precedence(&mut self) -> Precedence {
        get_precedence(self.lexer.get_current_token().unwrap().token)
    }

    /// Exists program with code: '1', which means application failure.
    pub fn throw_exception(&mut self, expected: Token, message: Option<String>) {
        let mut e = SyntaxError {
            error_line: self.lexer.get_line(self.lexer.current_line - 1),
            expected,
            got: self.lexer.current_token.clone().unwrap(),
            line: self.lexer.current_line,
            column: self.lexer.current_col
                - self.lexer.current_token.clone().unwrap().literal_len(),
            extra_message: message,
        };
        e.throw_exception();
    }
}

pub fn build_parser(lexer: Lexer) -> Parser {
    let mut p = Parser {
        lexer,
        prefix_parser: HashMap::new(),
        infix_parser: HashMap::new(),
        errors: Vec::new(),
    };

    // Prefix parsers
    p.add_prefix_parser(TokenType::Integer, parse_number_literal);
    p.add_prefix_parser(TokenType::Float, parse_number_literal);
    p.add_prefix_parser(TokenType::Minus, parse_negative_number);
    p.add_prefix_parser(TokenType::LeftParenthesis, parse_grouped_expression);
    p.add_prefix_parser(TokenType::Identifier, parse_identifier);
    p.add_prefix_parser(TokenType::True, parse_boolean);
    p.add_prefix_parser(TokenType::False, parse_boolean);
    p.add_prefix_parser(TokenType::InvertSign, parse_inverted_boolean);
    p.add_prefix_parser(TokenType::Function, parse_function);
    p.add_prefix_parser(TokenType::If, parse_conditional);
    p.add_prefix_parser(TokenType::Null, parse_expression_null);
    p.add_prefix_parser(TokenType::String, parse_string_literal);
    p.add_prefix_parser(TokenType::LeftBracket, parse_expression_array);
    p.add_prefix_parser(TokenType::For, parse_loop);
    p.add_prefix_parser(TokenType::LeftBrace, parse_expression_hashmap);

    // Infix parsers
    p.add_infix_parser(TokenType::Plus, parse_suffix_expression);
    p.add_infix_parser(TokenType::Power, parse_suffix_expression);
    p.add_infix_parser(TokenType::Multiply, parse_suffix_expression);
    p.add_infix_parser(TokenType::Divide, parse_suffix_expression);
    p.add_infix_parser(TokenType::Minus, parse_suffix_expression);
    p.add_infix_parser(TokenType::Modulo, parse_suffix_expression);
    p.add_infix_parser(TokenType::LeftParenthesis, parse_call);
    p.add_infix_parser(TokenType::Dot, parse_index_expression);
    p.add_infix_parser(TokenType::LeftBracket, parse_index_expression);
    p.add_infix_parser(TokenType::And, parse_suffix_expression);
    p.add_infix_parser(TokenType::Or, parse_suffix_expression);

    // Infix Parsers Comparisons
    p.add_infix_parser(TokenType::Equals, parse_suffix_expression);
    p.add_infix_parser(TokenType::RightArrow, parse_suffix_expression);
    p.add_infix_parser(TokenType::GreaterThanOrEquals, parse_suffix_expression);
    p.add_infix_parser(TokenType::LeftArrow, parse_suffix_expression);
    p.add_infix_parser(TokenType::LessThanOrEquals, parse_suffix_expression);
    p.add_infix_parser(TokenType::NotEquals, parse_suffix_expression);

    p
}
