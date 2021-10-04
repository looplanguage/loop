#[derive(Clone)]
pub struct Token {
    pub token: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Clone, Debug, AsRefStr, Copy, Eq, Hash)]
pub enum TokenType {
    Identifier,
    VariableDeclaration,
    Assign,
    Integer,
    Semicolon,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    AtSign,
    LeftParenthesis,
    RightParenthesis,
    AndSign,
    InvertSign,
    HashSign,
    DollarSign,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    Unknown,
    Eof,
}

pub fn create_token(token: TokenType, literal: String) -> Token {
    Token { token, literal }
}
