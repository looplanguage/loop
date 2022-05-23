use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Clone)]
pub enum ValueType {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    Array(Box<Vec<ValueType>>),
    Void,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    INT,
    FLOAT,
    BOOL,
    CHAR,
    ARRAY(Box<Type>),
    // Only allowed for function "return type"
    VOID,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Integer(int) => {
                write!(f, "INT {}", int)
            }
            ValueType::Boolean(bool) => {
                write!(f, "BOOL {}", bool)
            }
            ValueType::Array(arr) => {
                write!(f, "ARRAY {:?}", *arr.clone())
            }
            ValueType::Character(char) => {
                write!(f, "CHAR {}", char)
            }
            ValueType::Float(float) => {
                write!(f, "FLOAT {}", float)
            }
            _ => write!(f, "unknown type"),
        }
    }
}

impl Debug for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Integer(int) => {
                write!(f, "INT {}", int)
            }
            ValueType::Boolean(bool) => {
                write!(f, "BOOL {}", bool)
            }
            ValueType::Array(arr) => {
                write!(f, "ARRAY [{:?}]", *arr.clone())
            }
            ValueType::Character(char) => {
                write!(f, "CHAR {}", char)
            }
            _ => write!(f, "unknown type"),
        }
    }
}

impl ValueType {
    pub fn char_arr_to_string(&mut self) -> String {
        let mut s = String::new();
        if let ValueType::Array(arr) = self {
            let chars = *arr.clone();
            for c in chars {
                if let ValueType::Character(char) = c {
                    s.push(char);
                }
            }
            return s;
        }
        panic!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    }
}
