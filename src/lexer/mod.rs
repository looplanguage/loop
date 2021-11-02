mod test;
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
    pub fn next_token(&mut self) -> Token {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.internal_next_token());

        let cloned = self.current_token.clone();

        if cloned.is_none() {
            return create_token(TokenType::Unknown, "".to_string());
        }

        cloned.unwrap()
    }

    fn internal_next_token(&mut self) -> Token {
        let possible_char = self.input.chars().nth(self.current as usize);

        self.next_character();

        if possible_char == None {
            return create_token(TokenType::Eof, "".to_string());
        }

        let ch: char = possible_char.unwrap();

        if ch.is_whitespace() {
            return self.internal_next_token();
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
            ',' => create_token(TokenType::Comma, ch.to_string()),
            '{' => create_token(TokenType::LeftBrace, ch.to_string()),
            '}' => create_token(TokenType::RightBrace, ch.to_string()),
            '[' => create_token(TokenType::LeftBracket, ch.to_string()),
            ']' => create_token(TokenType::RightBracket, ch.to_string()),
            '.' => create_token(TokenType::Dot, ch.to_string()),
            '"' => self.find_string(),
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

    fn find_string(&mut self) -> Token {
        let mut string: String = String::new();
        self.next_character();

        while self.current_character() != '"' && self.current_character() != char::from(0) {
            string.push_str(self.current_character().to_string().as_str());
            self.next_character();
        }

        create_token(TokenType::String, string)
    }

    fn find_keyword(&mut self, ch: char) -> Token {
        let mut keyword: String = String::from(ch);

        while self.peek_character().is_alphanumeric() || self.peek_character() == '_' {
            keyword.push_str(self.peek_character().to_string().as_str());
            self.next_character();
        }

        let mut token_type: TokenType = lookup_keyword(keyword.as_str());

        if token_type == TokenType::Integer
            && self.peek_character() == '.'
            && self.double_peek_character().is_numeric()
        {
            keyword.push_str(self.peek_character().to_string().as_str());
            self.next_character();
            while self.peek_character().is_numeric() {
                keyword.push_str(self.peek_character().to_string().as_str());
                self.next_character();
            }
            token_type = TokenType::Float;
        }

        create_token(token_type, keyword.to_string())
    }

    fn peek_character(&self) -> char {
        let val = self.input.chars().nth((self.current) as usize);

        if val == None {
            return char::from(0);
        }

        val.unwrap()
    }

    fn double_peek_character(&self) -> char {
        let val = self.input.chars().nth((self.current + 1) as usize);

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
            self.next_token();
            return true;
        }

        false
    }

    pub fn next_current_is(&mut self, token: TokenType) -> bool {
        if self.current_token.clone().unwrap().token == token {
            self.next_token();
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
        "fn" => TokenType::Function,
        "import" => TokenType::Import,
        "export" => TokenType::Export,
        "else" => TokenType::Else,
        "for" => TokenType::For,
        "and" | "&&" => TokenType::And,
        "or" | "||" => TokenType::Or,
        "null" => TokenType::Null,
        "return" => TokenType::Return,
        "if" => TokenType::If,
        "as" => TokenType::As,
        "from" => TokenType::From,
        "to" => TokenType::To,
        _ => {
            if keyword.parse::<i64>().is_ok() {
                return TokenType::Integer;
            } else if keyword.parse::<f64>().is_ok() {
                return TokenType::Float;
            }
            if !keyword.contains('.') {
                return TokenType::Identifier;
            }
            panic!(
                "Error -> Keyword: {}, contains a '.', This is not allowed.",
                keyword
            )
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

    l.next_token();
    l.next_token();

    l
}
