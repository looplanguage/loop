mod test;
pub mod token;

use crate::lexer::token::create_token;
use std::borrow::Borrow;
use token::Token;
use token::TokenType;

pub struct Lexer {
    current: i32,
    input: String,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub current_line: i32,
    pub current_col: i32,
}

impl Lexer {
    pub fn next_token(&mut self) -> &Token {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.internal_next_token());

        if self.get_current_token().is_none() {
            self.current_token = Some(create_token(TokenType::Unknown, "".to_string()));
        }

        self.get_current_token().unwrap()
    }

    pub fn get_current_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn get_peek_token(&self) -> Option<&Token> {
        self.peek_token.as_ref()
    }

    pub fn get_line(&self, line: i32) -> String {
        let mut line_count = 0;
        let mut char_count = 0;
        for char in self.input.chars() {
            char_count += 1;
            if char == '\n' {
                line_count += 1;
            }
            if line_count == line {
                break;
            }
        }

        let mut line = String::from("");
        let mut current_char = self.input.chars().nth(char_count as usize);
        while !current_char.is_none() && current_char.unwrap() != '\n' {
            line.push(current_char.unwrap());
            char_count += 1;
            current_char = self.input.chars().nth(char_count as usize);
        }

        line
    }

    fn internal_next_token(&mut self) -> Token {
        let possible_char = self.input.chars().nth(self.current as usize);

        self.next_character();

        if possible_char.is_none() {
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
            ',' => create_token(TokenType::Comma, ch.to_string()),
            '{' => create_token(TokenType::LeftBrace, ch.to_string()),
            '}' => create_token(TokenType::RightBrace, ch.to_string()),
            '[' => create_token(TokenType::LeftBracket, ch.to_string()),
            ']' => create_token(TokenType::RightBracket, ch.to_string()),
            '.' => create_token(TokenType::Dot, ch.to_string()),
            ':' => create_token(TokenType::Colon, ch.to_string()),
            '^' => create_token(TokenType::Power, ch.to_string()),
            '"' => self.find_string(),
            '/' => {
                if self.peek_character() == '<' {
                    self.remove_block_comment();
                    self.internal_next_token()
                } else if self.peek_character() == '/' {
                    self.next_character();
                    self.remove_line_comment();
                    self.internal_next_token()
                } else {
                    create_token(TokenType::Divide, ch.to_string())
                }
            }
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
            let res: Option<String> = self.find_escape_sequence();
            if res != None {
                string.push_str(res.unwrap().as_str());
                self.next_character();
            } else {
                string.push_str(self.current_character().to_string().as_str());
            }
            self.next_character();
        }

        create_token(TokenType::String, string)
    }

    fn find_escape_sequence(&mut self) -> Option<String> {
        if self.current_character() != '\\' {
            return None;
        }

        match self.peek_character() {
            'n' => Some("\n".to_string()),
            't' => Some("\t".to_string()),
            'r' => Some("\r".to_string()),
            '\'' => Some("\'".to_string()),
            '\"' => Some("\"".to_string()),
            '\\' => Some("\\\\".to_string()),
            _ => None,
        }
    }

    fn find_keyword(&mut self, ch: char) -> Token {
        let mut keyword: String = String::from(ch);

        while self.peek_character().is_alphanumeric() || self.peek_character() == '_' {
            keyword.push_str(self.peek_character().to_string().as_str());
            self.next_character();
        }

        let mut token_type: TokenType = self.lookup_keyword(keyword.as_str());

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
        let possible_char = self.input.chars().nth(self.current as usize);
        if possible_char != None && self.current_character() == '\n' {
            self.current_line += 1;
            self.current_col = 0;
        } else {
            self.current_col += 1;
        }
    }

    fn current_character(&self) -> char {
        return self.input.chars().nth((self.current - 1) as usize).unwrap();
    }

    fn remove_line_comment(&mut self) {
        self.next_character();

        let start_index = self.current - 2;
        let mut end_index = start_index;
        let mut possible_char = self.input.chars().nth(self.current as usize);

        while possible_char != None && self.current_character() != '\n' {
            end_index += 1;
            possible_char = self.input.chars().nth(self.current as usize);
            self.next_character();
        }

        // Replacing the whole comment with spaces.
        // That way implementing line and column with an error is way easier.
        let mut replacement: String = "".to_string();
        for _ in 0..end_index - start_index {
            replacement.push(' ');
        }
        self.input.replace_range(
            (start_index) as usize..(end_index) as usize,
            replacement.as_str(),
        );
    }

    fn remove_block_comment(&mut self) {
        self.input
            .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
        self.next_character();
        self.input
            .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
        self.next_character();

        loop {
            let current: char = self.current_character();
            let next: char = self.peek_character();
            let possible_char = self.input.chars().nth(self.current as usize);

            if current == '\\' && next == 'n' {
                self.next_character();
                self.next_character();
            } else if current == '>' && next == '/' {
                self.input
                    .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
                self.next_character();
                self.input
                    .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
                break;
            } else if possible_char == None {
                break;
            } else {
                self.input
                    .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
                self.next_character();
            }
        }
    }

    pub fn next_is(&mut self, token: TokenType) -> bool {
        if let Some(peek_token) = self.get_peek_token() {
            if peek_token.token == token {
                self.next_token();
                return true;
            }
        }

        false
    }

    pub fn next_current_is(&mut self, token: TokenType) -> bool {
        if let Some(peek_token) = self.get_current_token() {
            if peek_token.token == token {
                self.next_token();
                return true;
            }
        }

        false
    }

    fn lookup_keyword(&mut self, keyword: &str) -> TokenType {
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
            "in" => TokenType::In,
            "to" => TokenType::To,
            _ => {
                if keyword.parse::<i64>().is_ok() {
                    return TokenType::Integer;
                } else if keyword.parse::<f64>().is_ok() {
                    return TokenType::Float;
                } else if !keyword.contains('.') {
                    return TokenType::Identifier;
                }
                // Not sure if you ever come to this error message, could not get it done...
                panic!(
                    "Error -> On line: {}, Column: {}. Message: Keyword: {}, contains a '.', This is not allowed.",
                    self.current_line,
                    self.current_col,
                    keyword,
                )
            }
        }
    }
}

pub fn build_lexer(input: &str) -> Lexer {
    let mut l = Lexer {
        current: 0,
        input: input.to_string(),
        current_token: None,
        peek_token: None,
        current_line: 1,
        current_col: 0,
    };

    l.next_token();
    l.next_token();

    l
}
