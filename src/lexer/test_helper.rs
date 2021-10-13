use crate::lexer::token::{Token, TokenType};

pub fn generate_token(literal: &str, token_type: TokenType) -> Token {
    return Token {
        token: token_type,
        literal: literal.to_string(),
    };
}
