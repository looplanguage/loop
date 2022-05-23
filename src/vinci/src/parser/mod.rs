use crate::ast::instructions::Node;
use crate::ast::AST;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use std::borrow::{Borrow, BorrowMut};

pub mod error;
mod instruction;
mod tests;

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl Parser<'_> {
    pub fn parse(&mut self) -> AST {
        let mut ast = AST::new();

        // Parse until at end
        while let Some(current) = self.lexer.borrow_mut().next() {
            let node = self.parse_node(&current);

            if let Ok(node) = node {
                ast.add_node(node);
            } else {
                // TODO: Clean this up a little
                let lexer = self.lexer.borrow();
                let source: &str = lexer.source();
                let span = lexer.span();
                println!("{}", source);
                let mut amount = String::from("");
                let mut spaces = String::from("");

                for _ in 0..span.start {
                    spaces.push(' ');
                }

                for _ in 0..(span.end - span.start) {
                    amount.push('^');
                }

                let source_str = String::from(source);

                let t = &source_str[span.start..span.end];

                println!("{}{}", spaces, amount);
                println!("Error: {}\n", node.expect_err("?"));
                println!("Near: {}\n", t);
                break;
            }
        }

        ast
    }

    /// Will parse nodes until a RightCurly token is found
    ///
    /// **Note:** Do not use `parser.expected(Token::RightCurly)` after calling this method
    /// this method will do that for you
    pub fn parse_nodes(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes: Vec<Node> = Vec::new();
        let mut current = self.next_token();

        while current != Token::RightCurly {
            nodes.push(self.parse_node(&current)?);
            current = self.next_token();
        }

        Ok(nodes)
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.lexer.borrow_mut().next() {
            token
        } else {
            Token::Error
        }
    }

    pub fn parse_node(&mut self, start: &Token) -> Result<Node, ParseError> {
        match start {
            Token::Instruction(ins) => {
                // Skip over the instruction, then parse it
                instruction::parse_instruction(self, ins.clone())
            }
            _ => Err(ParseError::Unknown),
        }
    }

    /// Get the current token, wrapper for lexer
    pub fn current_token(&self) -> Token {
        Token::End
    }

    /// Parser goes to next token, expects it to be the same as the parameter. If not it returns an exception
    pub fn expected(&mut self, token: Token) -> Result<Token, ParseError> {
        let next = self.next_token();
        if next != token {
            Err(ParseError::UnexpectedToken(token, self.current_token()))
        } else {
            Ok(next)
        }
    }

    /// Same as [expected] but never throws an error
    pub fn expected_maybe(&mut self, token: Token) -> bool {
        let last = self.lexer.borrow().clone().next();

        if last.unwrap() == token {
            self.lexer.next();
            return true;
        }

        false
    }

    pub fn new(lexer: logos::Lexer<Token>) -> Parser {
        Parser { lexer }
    }
}
