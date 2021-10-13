#[cfg(test)]
pub mod test_helper {
    use crate::lexer::token::{Token, TokenType};

    pub fn generate_token(literal: &str, token_type: TokenType) -> Token {
        Token {
            token: token_type,
            literal: literal.to_string(),
        }
    }
}
