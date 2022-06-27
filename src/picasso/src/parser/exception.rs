use crate::lexer::token::TokenType;

#[derive(Debug, Clone)]
pub enum Parenthesis {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum SyntaxException {
    Unknown,
    // Title, Description
    CustomMessage(String, Option<String>),
    ExpectedToken(TokenType),
    NoPrefixParser(TokenType),
    WrongParentheses(Parenthesis),
}
