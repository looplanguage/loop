pub struct Token {
    pub token: TokenType,
    pub literal: String
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(AsRefStr)]
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
    EOF
}

pub fn create_token(token: TokenType, literal: String) -> Token {
    return Token {
        token,
        literal: literal.to_string()
    }
}
