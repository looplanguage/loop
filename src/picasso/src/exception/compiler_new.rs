use colored::*;
use std::process;

/// Struct that contains all data needed to throw a parser error
/// # Example
/// ```(rust)
/// use picasso::exception::compiler_new::CompilerError;
/// let foo = CompilerError {
///    error_message: String::from("this is not allowed"),
///    extra_message: Some(String::from("Syntax -> if (<EXPRESSION>) {STATEMENTS} ")),
/// };
/// ```
#[allow(dead_code)]
pub struct CompilerError {
    pub error_message: String,
    pub extra_message: Option<String>,
}
#[allow(dead_code)]
impl CompilerError {
    #[rustfmt::skip]
    /// Prints parser error in terminal, and exits the program with code '1'
    /// # Error Template
    /// <pre>
    /// CompilerError -->
    ///    Message: a constant cannot be reassigned
    ///
    ///    NOTE:
    ///         (OPTIONAL NOTE)
    /// </pre>
    ///
    /// # Examples
    /// ```(rust)
    /// use picasso::exception::compiler_new::CompilerError;
    /// let mut foo = CompilerError { error_message: "".to_string(), extra_message: None };
    /// foo.throw_exception();
    /// ```
    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn throw_exception(&mut self) {
        println!("==========================================================");
        println!("{}", "CompilerError -->".bright_red());
        println!("    {}: {}", "Message".bright_blue(), format!("{}\n", self.error_message).bright_white());

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
    /// text = add_identation(4, text);
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
