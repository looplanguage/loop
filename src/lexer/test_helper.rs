use crate::lexer::token::{TokenType, Token};

pub fn generate_token(literal: &str, token_type: TokenType) -> Token{
    return Token {
        token: token_type,
        literal: literal.to_string()
    }
}