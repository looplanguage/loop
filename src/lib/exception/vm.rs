pub enum VMException {
    IncorrectArgumentCount(i32, i32),
    IncorrectType(String),
    CannotParseInt(String),
}
