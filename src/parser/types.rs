#[derive(Clone, PartialEq, Debug)]
pub enum BaseTypes {
    Integer,
    String,
    Boolean,
    Float,
    UserDefined(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Basic(BaseTypes),
    Array(BaseTypes),
    Function,
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
            Types::Auto => "auto".to_string(),
            Types::Function => "()".to_string(),
            Types::Void => "void".to_string(),
        }
    }
}
