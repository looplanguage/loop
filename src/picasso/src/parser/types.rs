use crate::parser::expression::Expression;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BaseTypes {
    Integer,
    String,
    Boolean,
    Float,
    UserDefined(String),
}

impl BaseTypes {
    pub fn transpile(&self) -> String {
        match self {
            BaseTypes::Integer => "INT".to_string(),
            BaseTypes::String => "STRING".to_string(),
            BaseTypes::Boolean => "BOOL".to_string(),
            BaseTypes::Float => "FLOAT".to_string(),
            BaseTypes::UserDefined(a) => a.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionType {
    pub return_type: Box<Types>,
    pub parameter_types: Vec<Types>,
    pub reference: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Library {
    pub methods: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClassItemType {
    pub name: String,
    pub index: u32,
    pub class_item_type: Types,
    pub value: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Compound(pub String, pub CompoundFields);

type CompoundFields = Box<Vec<ClassItemType>>;

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Basic(BaseTypes),
    Array(Box<Types>),
    // Return type & Parameter Types (for compile time)
    Function(FunctionType),
    Library(Library),
    Compound(Compound),
    Void,
    Auto,
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Types::Basic(basic) => match basic {
                    BaseTypes::Integer => "int".to_string(),
                    BaseTypes::String => "string".to_string(),
                    BaseTypes::Boolean => "bool".to_string(),
                    BaseTypes::Float => "float".to_string(),
                    BaseTypes::UserDefined(s) => s.to_string(),
                },
                Types::Array(array) => match *array.clone() {
                    Types::Basic(basic) => {
                        format!("{}[]", basic.transpile())
                    }
                    Types::Array(array) => {
                        format!("{}[][]", array.transpile())
                    }
                    Types::Function(_) => "()[]".to_string(),
                    Types::Void => "void[]".to_string(),
                    Types::Auto => "void[]".to_string(),
                    Types::Library(lib) => format!("LIBRARY {{{:?}}}", lib.methods),
                    Types::Compound(Compound(tp, _)) => tp,
                },
                Types::Auto => "Variant".to_string(),
                // TODO: Should probably be different now we know types
                Types::Function(func) => {
                    let mut args = String::new();

                    let mut index = 0;
                    for parameter_type in &func.parameter_types {
                        index += 1;
                        args.push_str(&*parameter_type.transpile());

                        if index != func.parameter_types.len() {
                            args.push(',');
                        }
                    }

                    format!("fn({}): {}", args, func.return_type)
                }
                Types::Compound(Compound(tp, _)) => tp.clone(),
                Types::Void => "void".to_string(),
                Types::Library(lib) => format!("LIBRARY {{{:?}}}", lib.methods),
            }
        )
    }
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
                Types::Compound(Compound(tp, _)) => tp,
            },
            Types::Auto => "Variant".to_string(),
            // TODO: Should probably be different now we know types
            Types::Function(_) => "VOID".to_string(),
            Types::Void => "VOID".to_string(),
            Types::Library(lib) => format!("LIBRARY {{{:?}}}", lib.methods),
            Types::Compound(Compound(tp, _)) => tp.clone(),
        }
    }
}
