#[derive(Clone, PartialEq, Debug)]
pub enum BaseTypes {
    Integer,
    String,
    Boolean,
    Float
}

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Basic(BaseTypes),
    Array(BaseTypes)
}

impl Types {
    pub fn transpile(&self) -> String {
        match self {
            Types::Basic(basic) => {
                match basic {
                    BaseTypes::Integer => "int".to_string(),
                    BaseTypes::String => "string".to_string(),
                    BaseTypes::Boolean => "bool".to_string(),
                    BaseTypes::Float => "float".to_string(),
                }
            }
            Types::Array(array) => {
                match array {
                    BaseTypes::Integer => "int[]".to_string(),
                    BaseTypes::String => "string[]".to_string(),
                    BaseTypes::Boolean => "bool[]".to_string(),
                    BaseTypes::Float => "float".to_string(),
                }
            }
        }
    }
}