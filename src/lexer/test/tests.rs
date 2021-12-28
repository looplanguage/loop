#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::lexer::test::test_helper::test_helper;
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn variable_declaration_numbers() {
        let input =
            "var test = 1; var _foo = 1; var bar = -1; var yeet = 1; var yeet2 = 0; var hello_world_2 = 3;";
        let expected = vec![
            // Statement 1
            test_helper::generate_token("var", TokenType::VariableDeclaration),
            test_helper::generate_token("test", TokenType::Identifier),
            test_helper::generate_token("=", TokenType::Assign),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            // Statement 2
            test_helper::generate_token("var", TokenType::VariableDeclaration),
            test_helper::generate_token("_foo", TokenType::Identifier),
            test_helper::generate_token("=", TokenType::Assign),
            test_helper::generate_token("1", TokenType::Integer), // CHANGE THIS BACK LATER (to 1.1 and Float)
            test_helper::generate_token(";", TokenType::Semicolon),
            // Statement 3
            test_helper::generate_token("var", TokenType::VariableDeclaration),
            test_helper::generate_token("bar", TokenType::Identifier),
            test_helper::generate_token("=", TokenType::Assign),
            test_helper::generate_token("-", TokenType::Minus),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            // Statement 4
            test_helper::generate_token("var", TokenType::VariableDeclaration),
            test_helper::generate_token("yeet", TokenType::Identifier),
            test_helper::generate_token("=", TokenType::Assign),
            test_helper::generate_token("1", TokenType::Integer), // CHANGE THIS BACK LATER (to 1.1 and Float)
            test_helper::generate_token(";", TokenType::Semicolon),
            // Statement 5
            test_helper::generate_token("var", TokenType::VariableDeclaration),
            test_helper::generate_token("yeet2", TokenType::Identifier),
            test_helper::generate_token("=", TokenType::Assign),
            //test_helper::generate_token("-", TokenType::Minus),
            test_helper::generate_token("0", TokenType::Integer), // CHANGE THIS BACK LATER (to -0.0001 and Float)
            test_helper::generate_token(";", TokenType::Semicolon),
            // Statement 6
            test_helper::generate_token("var", TokenType::VariableDeclaration),
            test_helper::generate_token("hello_world_2", TokenType::Identifier),
            test_helper::generate_token("=", TokenType::Assign),
            test_helper::generate_token("3", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
        ];

        do_test(input, expected);
    }

    #[test]
    fn escape_sequences() {
        let input = "\"x\\yx\" \"x\\ny\" \"x\\ty\" \"x\\ry\" \"x\\'y\" \"x\\\"y\" \"x\\\\y\"";
        let expected = vec![
            test_helper::generate_token("x\\yx", TokenType::String),
            test_helper::generate_token("x\ny", TokenType::String),
            test_helper::generate_token("x\ty", TokenType::String),
            test_helper::generate_token("x\ry", TokenType::String),
            test_helper::generate_token("x'y", TokenType::String),
            test_helper::generate_token("x\"y", TokenType::String),
            test_helper::generate_token("x\\\\y", TokenType::String),
        ];

        do_test(input, expected)
    }

    #[test]
    fn arithmetic_operations() {
        let input = "1 + 1; 1 - 5; 1 * 4; 20 / 3; 10 % 5; 2 ^ 3;";
        let expected = vec![
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token("+", TokenType::Plus),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token("-", TokenType::Minus),
            test_helper::generate_token("5", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token("*", TokenType::Multiply),
            test_helper::generate_token("4", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("20", TokenType::Integer),
            test_helper::generate_token("/", TokenType::Divide),
            test_helper::generate_token("3", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("10", TokenType::Integer),
            test_helper::generate_token("%", TokenType::Modulo),
            test_helper::generate_token("5", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("2", TokenType::Integer),
            test_helper::generate_token("^", TokenType::Power),
            test_helper::generate_token("3", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
        ];

        do_test(input, expected);
    }

    #[test]
    fn boolean_operations() {
        let input = "1 > 2; 1 >= 2; 1 < 2; 1 <= 2;";
        let expected = vec![
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token(">", TokenType::GreaterThan),
            test_helper::generate_token("2", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token(">=", TokenType::GreaterThanOrEquals),
            test_helper::generate_token("2", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token("<", TokenType::LessThan),
            test_helper::generate_token("2", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
            test_helper::generate_token("1", TokenType::Integer),
            test_helper::generate_token("<=", TokenType::LessThanOrEquals),
            test_helper::generate_token("2", TokenType::Integer),
            test_helper::generate_token(";", TokenType::Semicolon),
        ];

        do_test(input, expected);
    }

    #[test]
    fn strings() {
        let input = "\"hello\" \"world!\"";
        let expected = vec![
            test_helper::generate_token("hello", TokenType::String),
            test_helper::generate_token("world!", TokenType::String),
        ];

        do_test(input, expected);
    }

    #[test]
    fn strings_with_spaces() {
        let input = "\"hello world\" \"Even more spaces! And cool tokens :)\"";
        let expected = vec![
            test_helper::generate_token("hello world", TokenType::String),
            test_helper::generate_token("Even more spaces! And cool tokens :)", TokenType::String),
        ];

        do_test(input, expected);
    }

    #[test]
    fn comments() {
        let input = "/<hello2>/ //hello";
        let expected = vec![
            test_helper::generate_token("hello2", TokenType::Comment),
            test_helper::generate_token("hello", TokenType::Comment),
        ];

        do_test(input, expected);
    }

    fn do_test(input: &str, expected: Vec<Token>) {
        let mut l = lexer::build_lexer(input);
        let mut current_token = l.get_current_token().unwrap();

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
            l.next_token();
            current_token = l.get_current_token().unwrap();
        }
    }
}
