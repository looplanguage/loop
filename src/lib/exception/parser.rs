use crate::lexer::token::Token;
use colored::*;
use std::process;

// ToDo: Give some context with the error.
// ToDo: Give file in which error occurred.
pub struct ParserException {
    error_line: String,
    expected: Token,
    got: Token,
    line: i32,
    column: i32,
    extra_message: Option<String>,
}

impl ParserException {
    #[rustfmt::skip]
    pub fn throw_exception(&mut self) {
        println!("==========================================================");
        println!("{}", format!("SyntaxError [{}:{}] -->", self.line, self.column).bright_red());
        println!("    {}: {}", "Line".bright_blue(),format!("{}\n", self.error_line.as_str()).bright_white());
        println!("    {}: {}", "Message".bright_blue(), format!("Expected: '{}', but got: '{}'\n", self.expected.literal, self.got.literal).bright_white());

        if self.extra_message.is_none() {
            process::exit(0);
        }
        let message = self.add_identation(8, self.extra_message.as_ref().unwrap().to_string());
        println!("    {}:", "Note".bright_blue());
        println!("        {}", message.bright_white());
    }

    fn add_identation(&mut self, spaces: i32, text: String) -> String {
        let insert = " ".repeat(spaces as usize);
        let mut new_text: String = String::from("");
        for char in text.chars() {
            if char == 'o' {
                new_text += &*insert;
            }
            new_text.push(char);
        }
        return new_text.clone();
    }
}
