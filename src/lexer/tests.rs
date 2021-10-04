use crate::lexer;
use crate::lexer::token::{Token, TokenType};
#[cfg(test)]

#[test]
fn variable_declaration() {
    let input = "var test = 1;";
    let expected = vec![
        Token {
            token: TokenType::VariableDeclaration,
            literal: "var".to_string(),
        },
        Token {
            token: TokenType::Identifier,
            literal: "test".to_string(),
        },
        Token {
            token: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
    ];

    do_test(input, expected);
}

#[test]
fn arithmetic_operations() {
    let input = "1 + 1; 1 - 5; 1 * 4; 20 / 3; 10 % 5;";
    let expected = vec![
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::Plus,
            literal: "+".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::Minus,
            literal: "-".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "5".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::Multiply,
            literal: "*".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "4".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "20".to_string(),
        },
        Token {
            token: TokenType::Divide,
            literal: "/".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "3".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "10".to_string(),
        },
        Token {
            token: TokenType::Modulo,
            literal: "%".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "5".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
    ];

    do_test(input, expected);
}

#[test]
fn boolean_operations() {
    let input = "1 > 2; 1 >= 2; 1 < 2; 1 <= 2;";
    let expected = vec![
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::GreaterThan,
            literal: ">".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "2".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::GreaterThanOrEquals,
            literal: ">=".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "2".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::LessThan,
            literal: "<".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "2".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "1".to_string(),
        },
        Token {
            token: TokenType::LessThanOrEquals,
            literal: "<=".to_string(),
        },
        Token {
            token: TokenType::Integer,
            literal: "2".to_string(),
        },
        Token {
            token: TokenType::Semicolon,
            literal: ";".to_string(),
        },
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
