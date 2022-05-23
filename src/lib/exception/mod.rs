//! Exceptions that can be thrown by Loop
pub mod flag;

#[allow(dead_code)]
pub enum Exception {
    Syntax(String),
    NoHomeFolder,
}
