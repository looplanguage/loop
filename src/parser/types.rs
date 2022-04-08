#[derive(Clone, PartialEq, Debug)]
pub enum BaseTypes {
    Integer,
    String,
    Boolean,
    Float,
    UserDefined(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionType {
    pub return_type: Box<Types>,
    pub parameter_types: Vec<Box<Types>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Basic(BaseTypes),
    Array(BaseTypes),
    // Return type & Parameter Types (for compile time)
    Function(FunctionType),
    Void,
    Auto,
}

impl Types {
    pub fn transpile(&self) -> String {
        match self {
            Types::Basic(basic) => match basic {
                BaseTypes::Integer => "int".to_string(),
                BaseTypes::String => "string".to_string(),
                BaseTypes::Boolean => "bool".to_string(),
                BaseTypes::Float => "float".to_string(),
                BaseTypes::UserDefined(s) => s.to_string(),
            },
            Types::Array(array) => match array {
                BaseTypes::Integer => "int[]".to_string(),
                BaseTypes::String => "string[]".to_string(),
                BaseTypes::Boolean => "bool[]".to_string(),
                BaseTypes::Float => "float[]".to_string(),
                BaseTypes::UserDefined(s) => s.to_string(),
            },
            Types::Auto => "Variant".to_string(),
            // TODO: Should probably be different now we know types
            Types::Function(_) => "()".to_string(),
            Types::Void => "void".to_string(),
        }
    }
}
