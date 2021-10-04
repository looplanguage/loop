mod tests;
pub mod token;

use crate::lexer::token::create_token;
use token::Token;
use token::TokenType;

pub struct Lexer {
    current: i32,
    input: String,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Lexer {
    pub fn next(&mut self) -> Token {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.next_token());

        let cloned = self.current_token.clone();

        if cloned.is_none() {
            return create_token(TokenType::Unknown, "".to_string());
        }

        cloned.unwrap()
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
            return true;
        }

        false
    }
}

fn lookup_keyword(keyword: &str) -> TokenType {
    match keyword {
        "var" => TokenType::VariableDeclaration,
        "true" => TokenType::True,
        "false" => TokenType::False,
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

    l
}
