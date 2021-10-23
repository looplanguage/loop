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
    Float,
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
    True,
    False,
    Function,
    If,
    Else,
    While,
    Import,
    Export,
    And,
    Or,
    Comma,
    LeftBrace,
    RightBrace,
    Null,
    Return,
    Dot,
    LeftBracket,
    RightBracket,
    String,
    Eof,
}

pub fn create_token(token: TokenType, literal: String) -> Token {
    Token { token, literal }
}
