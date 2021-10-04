pub mod token;

use crate::lexer::token::create_token;
use token::Token;
use token::TokenType;

pub struct Lexer {
    current: i32,
    input: String,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>
}

impl Lexer {
    pub fn next(&mut self) -> Token {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.next_token());

        let cloned = self.current_token.clone();

        if cloned.is_none() {
            return create_token(TokenType::Unknown, "".to_string())
        }

        return cloned.unwrap();
    }

    fn next_token(&mut self) -> Token {
        let possible_char = self.input.chars().nth(self.current as usize);

        self.next_character();

        if possible_char == None {
            return create_token(TokenType::Eof, "".to_string());
        }

        let ch: char = possible_char.unwrap();

        if ch.is_whitespace() {
            return self.next_token();
        }

        match ch {
            ';' => create_token(TokenType::Semicolon, ch.to_string()),
            '+' => create_token(TokenType::Plus, ch.to_string()),
            '@' => create_token(TokenType::AtSign, ch.to_string()),
            '#' => create_token(TokenType::HashSign, ch.to_string()),
            '$' => create_token(TokenType::DollarSign, ch.to_string()),
            '%' => create_token(TokenType::Modulo, ch.to_string()),
            '&' => create_token(TokenType::AndSign, ch.to_string()),
            '*' => create_token(TokenType::Multiply, ch.to_string()),
            '(' => create_token(TokenType::LeftParenthesis, ch.to_string()),
            ')' => create_token(TokenType::RightParenthesis, ch.to_string()),
            '-' => create_token(TokenType::Minus, ch.to_string()),
            '/' => create_token(TokenType::Divide, ch.to_string()),
            '!' => {
                if self.peek_character() == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::NotEquals,
                        ch.to_string() + self.current_character().to_string().as_str(),
                    );
                }

                create_token(TokenType::InvertSign, ch.to_string())
            }
            '=' => {
                if self.peek_character() == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::Equals,
                        ch.to_string() + self.current_character().to_string().as_str(),
                    );
                }

                create_token(TokenType::Assign, ch.to_string())
            }
            '>' => {
                if self.peek_character() == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::GreaterThanOrEquals,
                        ch.to_string() + self.current_character().to_string().as_str(),
                    );
                }

                create_token(TokenType::GreaterThan, ch.to_string())
            }
            '<' => {
                if self.peek_character() == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::LessThanOrEquals,
                        ch.to_string() + self.current_character().to_string().as_str(),
                    );
                }

                create_token(TokenType::LessThan, ch.to_string())
            }
            _ => self.find_keyword(ch),
        }
    }

    fn find_keyword(&mut self, ch: char) -> Token {
        let mut keyword: String = String::from(ch);

        while self.peek_character().is_alphanumeric() {
            keyword.push_str(self.peek_character().to_string().as_str());
            self.current += 1;
        }

        let token_type: TokenType = lookup_keyword(keyword.as_str());

        create_token(token_type, keyword.to_string())
    }

    fn peek_character(&self) -> char {
        let val = self.input.chars().nth((self.current) as usize);

        if val == None {
            return char::from(0);
        }

        val.unwrap()
    }

    fn next_character(&mut self) {
        self.current += 1;
    }

    fn current_character(&self) -> char {
        return self.input.chars().nth((self.current - 1) as usize).unwrap();
    }

    pub fn next_is(&mut self, token: TokenType) -> bool {
        if self.peek_token.clone().unwrap().token == token {
            self.next();
            return true
        }

        return false
    }

    fn skip_whitespaces(&mut self) {
        if self.current_character().is_whitespace() {
            self.next_character();
            self.skip_whitespaces()
        }
    }
}

fn lookup_keyword(keyword: &str) -> TokenType {
    match keyword {
        "var" => TokenType::VariableDeclaration,
        _ => {
            if keyword.parse::<i32>().is_ok() {
                return TokenType::Integer;
            }

            TokenType::Identifier
        }
    }
}

pub fn build_lexer(input: &str) -> Lexer {
    let mut l = Lexer {
        current: 0,
        input: input.to_string(),
        current_token: None,
        peek_token: None,
    };

    l.next();
    l.next();

    return l
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::lexer::token::{Token, TokenType};

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
}
