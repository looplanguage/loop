#[derive(Clone)]
pub struct Token {
    pub token: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Clone, Debug, AsRefStr, Copy, Eq, Hash)]
pub enum TokenType {
    /// Variable name
    Identifier,
    /// Example: 'var'
    VariableDeclaration,
    /// Example: Positive and negative number without decimal
    Integer,
    /// Example: Positive number with decimal
    Float,
    /// Example: Negative number with decimal
    MinusFloat,
    /// Example:  ';'
    Semicolon,
    /// Example: '+'
    Plus,
    /// Example: '-'
    Minus,
    /// Example: '*'
    Multiply,
    /// Example: '='
    Assign,
    /// Example: '/'
    Divide,
    /// Example: '%'
    Modulo,
    /// Example: '^'
    Power,
    /// Example: '@'
    AtSign,
    /// Example: '('
    LeftParenthesis,
    /// Example: ')'
    RightParenthesis,
    /// Example: '&'
    AndSign,
    /// Example: '!'
    InvertSign,
    /// Example: '#'
    HashSign,
    /// Example: '$'
    DollarSign,
    /// Example: '>'
    GreaterThan,
    /// Example: '<'
    LessThan,
    /// Example: ','
    Comma,
    /// Example: '{'
    LeftBrace,
    /// Example: '}'
    RightBrace,
    /// Example: '.'
    Dot,
    /// Example: '['
    LeftBracket,
    /// Example: ']'
    RightBracket,
    /// Example: ':'
    Colon,
    /// Example: '=='
    Equals,
    /// Example: '!='
    NotEquals,
    /// Example: '>='
    GreaterThanOrEquals,
    /// Example: '<='
    LessThanOrEquals,
    /// Example: 'true' value
    True,
    /// Example: 'false' value
    False,
    /// Example: 'fn'
    Function,
    /// Example: 'if'
    If,
    /// Example: 'else'
    Else,
    /// Example: 'import'
    Import,
    /// Example: 'export'
    Export,
    /// Example: '&&' and 'and'
    And,
    /// Example: '||' and 'or'
    Or,
    /// Example: 'null' value
    Null,
    /// Example: 'return'
    Return,
    /// Description: A piece of text
    ///
    /// Example: "I am a piece of text"
    String,
    /// Example: 'as'
    As,
    /// Example: 'for'
    For,
    /// Example: 'from'
    From,
    /// Example: 'to'
    To,
    /// Example: 'in'
    In,
    /// Example: 'break'
    Break,
    Unknown,
    /// Indicates that there are no more tokens
    Eof,
}

pub fn create_token(token: TokenType, literal: String) -> Token {
    Token { token, literal }
}
