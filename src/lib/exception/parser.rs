use crate::lexer::token::Token;
use colored::*;
use std::process;

/// Struct that contains all data needed to throw a parser error
/// # Example
/// ```(rust)
/// let foo = ParserException {
///    error_line: String::from("if (true) }"),
///    expected: Token {TokenType::LeftBracket, String::from("{")},
///    got: Token {TokenType::RightBracket, String::from("}")},
///    line: 3,
///    column: 23,
///    extra_message: Some(String::from("Syntax -> if (<EXPRESSION>) {STATEMENTS} ")),
/// }
/// ```
pub struct ParserException {
    error_line: String,
    expected: Token,
    got: Token,
    line: i32,
    column: i32,
    extra_message: Option<String>,
}

impl ParserException {

    /// Prints parser error in terminal, and exits the program with code '1'
    /// # Examples
    /// ```(rust)
    /// let foo = ParserException { /*Instantiate ParserException Struct*/ };
    /// foo.throw_exception();
    /// ```
    #[rustfmt::skip]
    pub fn throw_exception(&mut self) {
        println!("==========================================================");
        println!("{}", format!("SyntaxError [{}:{}] -->", self.line, self.column).bright_red());
        println!("    {}: {}", "Line".bright_blue(),format!("{}\n", self.error_line.as_str()).bright_white());
        println!("    {}: {}", "Message".bright_blue(), format!("Expected: '{}', but got: '{}'\n", self.expected.literal, self.got.literal).bright_white());

        if self.extra_message.is_none() {
            process::exit(1);
        }
        let message = self.add_identation(8, self.extra_message.as_ref().unwrap().to_string());
        println!("    {}:", "Note".bright_blue());
        println!("        {}", message.bright_white());

        process::exit(1);
    }

    /// Inserts spaces into string before every '\n'
    /// # Example
    /// ```(rust)
    /// let mut text = String::from("Hello,
    /// World!");
    /// text = self.add_identation(4, text);
    /// ```
    /// <pre>
    /// Before:  "Hello,\nWorld!"
    /// After:   "Hello,\n    World!"
    /// </pre>
    fn add_identation(&mut self, spaces: i32, text: String) -> String {
        let insert = " ".repeat(spaces as usize);
        let mut new_text: String = String::from("");
        for char in text.chars() {
            if char == '\n' {
                new_text.push(char);
                new_text += &*insert;
            }
            else {
                new_text.push(char);
            }
        }
        return new_text.clone();
    }
}
