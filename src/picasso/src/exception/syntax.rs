use crate::lexer::token::Token;
use colored::*;
use std::process;

/// Struct that contains all data needed to throw a parser error
/// # Example
/// ```(rust)
/// let foo = SyntaxError {
///    error_line: String::from("if (true) }"),
///    expected: Token {TokenType::LeftBracket, String::from("{")},
///    got: Token {TokenType::RightBracket, String::from("}")},
///    line: 3,
///    column: 23,
///    extra_message: Some(String::from("Syntax -> if (<EXPRESSION>) {STATEMENTS} ")),
/// }
/// ```
#[allow(dead_code)]
pub struct SyntaxError {
    pub error_line: String,
    pub expected: Token,
    pub got: Token,
    pub line: i32,
    pub column: i32,
    pub extra_message: Option<String>,
}
#[allow(dead_code)]
impl SyntaxError {
    #[rustfmt::skip]
    /// Prints parser error in terminal, and exits the program with code '1'
    /// # Error Template
    /// <pre>
    /// SyntaxError [(LINE):(COLUMN)] -->
    ///    Error: (LINE OF ERROR)
    ///
    ///    Expected: '(TOKEN)', but got: '(TOKEN)'
    ///
    ///    NOTE:
    ///         (OPTIONAL NOTE)
    /// </pre>
    ///
    /// # Examples
    /// ```(rust)
    /// let foo = SyntaxError { /*Instantiate SyntaxError Struct*/ };
    /// foo.throw_exception();
    /// ```
    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn throw_exception(&mut self) {
        println!("==========================================================");
        println!("{}", format!("SyntaxError [{}:{}] -->", self.line, self.column).bright_red());
        println!("{}", self.error_line.as_str());

        let mut index = 1;
        while index < self.column {
            index += 1;
            print!(" ");
        }

        let mut amount = 1;
        loop {
            print!("^");
            amount += 1;
            if amount > self.got.literal.len() {
                break;
            }
        }

        println!();

        println!("    {}: {}", "Message".bright_blue(), format!("Expected: '{:?}', but got: '{}'\n", self.expected, self.got.literal).bright_white());

        if self.extra_message.is_none() {
            #[cfg(test)]
            panic!("See above!");

            process::exit(1);
        }
        let message = self.add_identation(8, self.extra_message.as_ref().unwrap().to_string());
        println!("    {}:", "Note".bright_blue());
        println!("        {}", message.bright_white());

        #[cfg(test)]
        panic!("See above!");

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
            new_text.push(char);
            if char == '\n' {
                new_text += &*insert;
            }
        }
        new_text.clone()
    }
}

pub fn throw_syntax_error(line_num: i32, column_num: i32, error_line: String, string: String) {
    println!("==========================================================");
    println!(
        "{}",
        format!("SyntaxError [{}:{}] -->", line_num, column_num).bright_red()
    );
    println!(
        "    {}: {}",
        "Line".bright_blue(),
        format!("{}\n", error_line.as_str()).bright_white()
    );
    println!("    Unexpected string: {}", string.as_str().bright_white());
    process::exit(1);
}
