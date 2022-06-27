use crate::lexer::token::TokenType;

#[derive(Debug)]
pub enum Parenthesis {
    Left,
    Right
}

#[derive(Debug)]
pub enum SyntaxException {
    Unknown,
    CustomMessage(String),
    ExpectedToken(TokenType),
    NoPrefixParser(TokenType),
    WrongParentheses(Parenthesis)
}