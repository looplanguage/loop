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
    MinusFloat,
    Multiply,
    Divide,
    Modulo,
    Power,
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
    As,
    Comment,
    For,
    From,
    To,
    In,
    Colon,
    Break,
    Eof,
}

pub fn create_token(token: TokenType, literal: String) -> Token {
    Token { token, literal }
}
