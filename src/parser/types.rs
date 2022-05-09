use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
pub enum BaseTypes {
    Integer,
    String,
    Boolean,
    Float,
    UserDefined(String),
    Null,
}

impl BaseTypes {
    pub fn transpile(&self) -> String {
        match self {
            BaseTypes::Integer => "INT".to_string(),
            BaseTypes::String => "STRING".to_string(),
            BaseTypes::Boolean => "BOOL".to_string(),
            BaseTypes::Float => "FLOAT".to_string(),
            BaseTypes::UserDefined(a) => a.clone(),
            BaseTypes::Null => "VOID".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionType {
    pub return_type: Box<Types>,
    pub parameter_types: Vec<Box<Types>>,
    pub reference: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Library {
    pub methods: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Basic(BaseTypes),
    Array(Box<Types>),
    // Return type & Parameter Types (for compile time)
    Function(FunctionType),
    Library(Library),
    Void,
    Auto,
}

impl Types {
    pub fn transpile(&self) -> String {
        match self {
            Types::Basic(basic) => match basic {
                BaseTypes::Integer => "INT".to_string(),
                BaseTypes::String => "CHAR[]".to_string(),
                BaseTypes::Boolean => "BOOL".to_string(),
                BaseTypes::Float => "FLOAT".to_string(),
                BaseTypes::UserDefined(s) => s.to_string(),
                BaseTypes::Null => "VOID".to_string(),
            },
            Types::Array(array) => match *array.clone() {
                Types::Basic(basic) => {
                    format!("{}[]", basic.transpile())
                }
                Types::Array(array) => {
                    format!("{}[][]", array.transpile())
                }
                Types::Function(_) => "()[]".to_string(),
                Types::Void => "VOID[]".to_string(),
                Types::Auto => "VOID[]".to_string(),
                Types::Library(lib) => format!("LIBRARY {{{:?}}}", lib.methods),
            },
            Types::Auto => "Variant".to_string(),
            // TODO: Should probably be different now we know types
            Types::Function(_) => "auto".to_string(),
            Types::Void => "VOID".to_string(),
            Types::Library(lib) => format!("LIBRARY {{{:?}}}", lib.methods),
        }
    }
}
