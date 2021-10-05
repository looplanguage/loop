#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn variable_declaration() {
        let input = "var test = 1;";
        let expected = vec![
            generate_token("var", TokenType::VariableDeclaration),
            generate_token("test", TokenType::Identifier),
            generate_token("=", TokenType::Assign),
            generate_token("1", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
        ];

        do_test(input, expected);
    }

    #[test]
    fn arithmetic_operations() {
        let input = "1 + 1; 1 - 5; 1 * 4; 20 / 3; 10 % 5;";
        let expected = vec![
            generate_token("1", TokenType::Integer),
            generate_token("+", TokenType::Plus),
            generate_token("1", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("1", TokenType::Integer),
            generate_token("-", TokenType::Minus),
            generate_token("5", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("1", TokenType::Integer),
            generate_token("*", TokenType::Multiply),
            generate_token("4", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("20", TokenType::Integer),
            generate_token("/", TokenType::Divide),
            generate_token("3", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("10", TokenType::Integer),
            generate_token("%", TokenType::Modulo),
            generate_token("5", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
        ];

        do_test(input, expected);
    }

    #[test]
    fn boolean_operations() {
        let input = "1 > 2; 1 >= 2; 1 < 2; 1 <= 2;";
        let expected = vec![
            generate_token("1", TokenType::Integer),
            generate_token(">", TokenType::GreaterThan),
            generate_token("2", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("1", TokenType::Integer),
            generate_token(">=", TokenType::GreaterThanOrEquals),
            generate_token("2", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("1", TokenType::Integer),
            generate_token("<", TokenType::LessThan),
            generate_token("2", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
            generate_token("1", TokenType::Integer),
            generate_token("<=", TokenType::LessThanOrEquals),
            generate_token("2", TokenType::Integer),
            generate_token(";", TokenType::Semicolon),
        ];

        do_test(input, expected);
    }

    fn do_test(input: &str, expected: Vec<Token>) {
        let mut l = lexer::build_lexer(input);
        let mut current_token: Token = l.current_token.clone().unwrap();

        let mut i = 0;
        while current_token.token != TokenType::Eof {
            assert_eq!(
                current_token.token,
                expected[i].token,
                "wrong token. got={}. expected={} (values {} & {})",
                current_token.token.as_ref(),
                expected[i].token.as_ref(),
                current_token.literal,
                expected[i].literal
            );
            assert_eq!(
                current_token.literal,
                expected[i].literal,
                "wrong value. got={}. expected={} (tokens {} & {})",
                current_token.literal,
                expected[i].literal,
                current_token.token.as_ref(),
                expected[i].token.as_ref()
            );

            i = i + 1;
            l.next();
            current_token = l.current_token.clone().unwrap();
        }
    }

    fn generate_token(literal: &str, token_type: TokenType) -> Token{
        return Token {
            token: token_type,
            literal: literal.to_string()
        }
    }
}
