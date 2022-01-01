use crate::lexer::token::Token;
use colored::*;
use std::process;

// ToDo: Give some context with the error.
// ToDo: Give file in which error occurred.
pub struct ParserException {
    expected: Token,
    got: Token,
    line: i32,
    column: i32,
    extra_message: Option<String>,
}

impl ParserException {
    pub fn throw_exception(&mut self) {
        let type_exc = "SyntaxError";
        let stripe = "==========================================";

        let title = format!(
            "{} [{}:{}]\n\n",
            type_exc.bright_red(),
            self.line,
            self.column
        );
        let error = format!(
            "   Expected: \'{}\', but got: \'{}\'\n",
            self.expected.literal, self.got.literal
        );

        let mut note: String = "".to_string();
        if self.extra_message.is_some() {
            note = format!("    {}\n", self.extra_message.unwrap())
        }

        println!(stripe);
        println!(title);
        println!(error);
        println!(note);
        println!(stripe);
        process::exit(0)
    }
}
