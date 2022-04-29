//! Responsible for lexing input string into tokens
mod test;
pub mod token;

use crate::lexer::token::create_token;
use crate::exception::syntax::throw_syntax_error;
use token::Token;
use token::TokenType;

/// The Lexer itself, containing metadata needed during the lexing process
pub struct Lexer {
    current: i32,
    input: String,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub current_line: i32,
    pub current_col: i32,
}

impl Lexer {
    /// Converts the next piece of the input into a Token.
    /// Call: `self.get_current_token()`, to get the tokenized token.
    ///
    /// returns: &Token
    ///
    /// # Examples
    ///
    /// ```
    /// let token: Token = self.next_token();
    /// ```
    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.internal_next_token());

        if self.get_current_token().is_none() {
            self.current_token = Some(create_token(TokenType::Unknown, "".to_string()));
        }
    }

    pub fn get_current_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn get_peek_token(&self) -> Option<&Token> {
        self.peek_token.as_ref()
    }

    /// Returns current line of the lexer as a String.
    ///
    /// It is used to throw syntax errors.
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
        while current_char.is_some() && current_char.unwrap() != '\n' {
            line.push(current_char.unwrap());
            char_count += 1;
            current_char = self.input.chars().nth(char_count as usize);
        }

        line.trim_start().to_string()
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
                if self.get_character(1) == '<' {
                    self.remove_block_comment();
                    self.internal_next_token()
                } else if self.get_character(1) == '/' {
                    self.next_character();
                    self.remove_line_comment();
                    self.internal_next_token()
                } else {
                    create_token(TokenType::Divide, ch.to_string())
                }
            }
            '!' => {
                if self.get_character(1) == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::NotEquals,
                        ch.to_string() + self.get_character(0).to_string().as_str(),
                    );
                }

                create_token(TokenType::InvertSign, ch.to_string())
            }
            '=' => {
                if self.get_character(1) == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::Equals,
                        ch.to_string() + self.get_character(0).to_string().as_str(),
                    );
                }

                create_token(TokenType::Assign, ch.to_string())
            }
            '>' => {
                if self.get_character(1) == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::GreaterThanOrEquals,
                        ch.to_string() + self.get_character(0).to_string().as_str(),
                    );
                }

                create_token(TokenType::GreaterThan, ch.to_string())
            }
            '<' => {
                if self.get_character(1) == '=' {
                    self.next_character();
                    return create_token(
                        TokenType::LessThanOrEquals,
                        ch.to_string() + self.get_character(0).to_string().as_str(),
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

        while self.get_character(0) != '"' && self.get_character(0) != char::from(0) {
            let res: Option<String> = self.find_escape_sequence();
            if res != None {
                string.push_str(res.unwrap().as_str());
                self.next_character();
            } else {
                string.push_str(self.get_character(0).to_string().as_str());
            }
            self.next_character();
        }

        create_token(TokenType::String, string)
    }

    fn find_escape_sequence(&mut self) -> Option<String> {
        if self.get_character(0) != '\\' {
            return None;
        }

        match self.get_character(1) {
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

        while self.get_character(1).is_alphanumeric() || self.get_character(1) == '_' {
            keyword.push_str(self.get_character(1).to_string().as_str());
            self.next_character();
        }

        let mut token_type: TokenType = self.lookup_keyword(keyword.as_str());

        if token_type == TokenType::Integer
            && self.get_character(1) == '.'
            && self.get_character(2).is_numeric()
        {
            keyword.push_str(self.get_character(1).to_string().as_str());
            self.next_character();
            while self.get_character(1).is_numeric() {
                keyword.push_str(self.get_character(1).to_string().as_str());
                self.next_character();
            }
            token_type = TokenType::Float;
        }

        create_token(token_type, keyword.to_string())
    }

    /// Lexer increments to next character of its input.
    ///
    /// **Note:** Also increments the column-and-line counter correct, and resets column counts if necessary.
    fn next_character(&mut self) {
        self.current += 1;

        // Keeping the line and column counter correct for the syntax error handeling
        let possible_char = self.input.chars().nth(self.current as usize);
        if possible_char != None && self.get_character(0) == '\n' {
            self.current_line += 1;
            self.current_col = 0;
        } else {
            self.current_col += 1;
        }
    }

    /// Returns a character of the lexer, `diff` allows you to "peek" at the upcoming characters
    ///
    /// # Arguments
    ///
    /// * `diff`: How far you want to look at the upcoming chars
    ///
    /// returns: char
    ///
    /// # Examples
    ///
    /// ```
    /// self.get_character(0);  // Returns current character
    /// self.get_character(1);  // "Peeking" at the next character
    /// self.get_character(-1); // Getting the previous character
    /// ```
    fn get_character(&self, diff: i32) -> char {
        let val = self.input.chars().nth((self.current + diff - 1) as usize);

        if val == None {
            return char::from(0);
        }

        val.unwrap()
    }

    /// Replaces the line comment in the inputted source code with spaces.
    /// It stops when it reaches the end of the line: '\n'
    ///
    /// **Note:** It is replaced with spaces, to keep the location of errors working.
    fn remove_line_comment(&mut self) {
        self.next_character();

        let start_index = self.current - 2;
        let mut end_index = start_index;
        let mut possible_char = self.input.chars().nth(self.current as usize);

        while possible_char != None && self.get_character(0) != '\n' {
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

    /// Replaces the block comment in the inputted source code with spaces.
    /// It stops when it reaches the closing token of a block comments ("\>")
    ///
    /// **Note:** It is replaced with spaces, to keep the location of errors working.
    fn remove_block_comment(&mut self) {
        self.input
            .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
        self.next_character();
        self.input
            .replace_range((self.current - 1) as usize..(self.current) as usize, " ");
        self.next_character();

        loop {
            let current: char = self.get_character(0);
            let next: char = self.get_character(1);
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

    /// Checks if next token is the same as the current token,
    /// **if next and given token are the same, then**: `lexer::next_token()`
    pub fn next_token_is_and_next_token(&mut self, token: TokenType) -> bool {
        if let Some(peek_token) = self.get_peek_token() {
            if peek_token.token == token {
                self.next_token();
                return true;
            }
        }

        false
    }

    /// Checks if given TokenType is the same as the current token,
    /// **if current and given token are the same, then**: `lexer::next_token()`
    pub fn next_token_and_current_is(&mut self, token: TokenType) -> bool {
        if let Some(peek_token) = self.get_current_token() {
            if peek_token.token == token {
                self.next_token();
                return true;
            }
        }

        false
    }

    /// Returns the type of the token, of any literal that is larger than one character
    fn lookup_keyword(&self, keyword: &str) -> TokenType {
        match keyword {
            "var" => TokenType::VariableDeclaration,
            "const" => TokenType::ConstantDeclaration,
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
            "break" => TokenType::Break,
            _ => {
                if keyword.parse::<i64>().is_ok() {
                    return TokenType::Integer;
                } else if keyword.parse::<f64>().is_ok() {
                    return TokenType::Float;
                }
                if !keyword.contains('.') {
                    return TokenType::Identifier;
                }
                throw_syntax_error(
                    self.current_line,
                    self.current_col,
                    self.get_line(self.current_line),
                    keyword.to_string(),
                );
                // Will never be reached, throw_function_error will quit program before.
                TokenType::Unknown
            }
        }
    }
}

/// Creates a new instance of the Lexer with the input string provided
///
/// # Examples
/// ```
/// let new_lexer = build_lexer("var x = 100");
/// ```
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
