use crate::ast::instructions::memory::LoadType;
use crate::types::Type;
use logos::{Lexer, Logos};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(PartialEq, Debug, Clone, EnumString)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    CONSTANT,
    LOAD,
    STORE,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    POWER,
    IF,
    FUNCTION,
    CALL,
    WHILE,
    GREATERTHAN,
    EQUALS,
    NOTEQUALS,
    PUSH,
    INDEX,
    SLICE,
    COPY,
    LOADLIB,
    RETURN,
    ASSIGN,
    POP,
    LENGTH,
    AND,
    OR,
    MODULO,
}

impl Instruction {
    pub fn find_instruction(str: &str) -> Option<Instruction> {
        let found = Instruction::from_str(str);

        if let Ok(found) = found {
            Some(found)
        } else {
            None
        }
    }
}

fn instruction(lex: &mut Lexer<Token>) -> Option<Instruction> {
    let mut string = lex.slice().to_string();
    string.remove(0);
    Instruction::find_instruction(string.as_str())
}

fn type_lex(lex: &mut Lexer<Token>) -> Option<Type> {
    let str = lex.slice().to_string();
    let typ = str.as_str().split("[]").next().unwrap();
    let array_occurences = str.matches("[]").count();

    let _type = match typ {
        "INT" => Some(Type::INT),
        "BOOL" => Some(Type::BOOL),
        "CHAR" => Some(Type::CHAR),
        "VOID" => Some(Type::VOID),
        "FLOAT" => Some(Type::FLOAT),
        _ => None,
    };

    if array_occurences == 0 {
        _type
    } else {
        Some(get_nested_array_type(_type.unwrap(), array_occurences - 1))
    }
}

fn get_nested_array_type(_type: Type, occuring: usize) -> Type {
    if occuring > 0 {
        return get_nested_array_type(Type::ARRAY(Box::new(_type)), occuring - 1);
    }

    Type::ARRAY(Box::new(_type))
}

fn load_type_lex(lex: &mut Lexer<Token>) -> Option<LoadType> {
    LoadType::find_type(lex.slice().to_string().as_str())
}

fn lex_string_array(lex: &mut Lexer<Token>) -> Vec<char> {
    let mut vec: Vec<char> = lex.slice().to_string().chars().collect();

    // Remove the double quotes
    vec.remove(0);
    vec.remove(vec.len() - 1);

    vec
}
fn lex_char(lex: &mut Lexer<Token>) -> char {
    let vec: Vec<char> = lex.slice().to_string().chars().collect();

    *vec.get(1).unwrap()
}

#[derive(PartialEq, Debug, Clone, Logos)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
    #[regex(r"\.\S+", instruction)]
    Instruction(Instruction),
    #[regex(r"PARAMETER|VARIABLE", load_type_lex)]
    LoadType(LoadType),
    #[token(";")]
    Semicolon,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, lex_string_array)]
    String(Vec<char>),
    #[regex(r#"'.'"#, lex_char)]
    Character(char),
    #[token("CONDITION")]
    Condition,
    #[token("THEN")]
    Then,
    #[token("ELSE")]
    Else,
    #[token("FREE")]
    Free,
    #[regex(r"(INT|BOOL|CHAR|VOID|FLOAT)(\[\])*", type_lex)]
    Type(Type),
    #[regex("[+-]?[0-9]+[.][0-9]+", |lex| lex.slice().parse())]
    Float(f64),
    #[regex("[+-]?[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
    #[regex(r"true|false", |lex| lex.slice().parse())]
    Boolean(bool),
    #[token("ARGUMENTS")]
    Arguments,
    #[regex(r"[a-zA-Z0-9_-]+::[a-zA-Z0-9_-]+", |lex| lex.slice().to_string())]
    Namespace(String),
    #[token("WHILE")]
    While,
    End,
}
