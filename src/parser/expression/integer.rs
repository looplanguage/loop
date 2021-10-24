use crate::parser::expression::string::LoopString;
use crate::parser::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn find_extension(&self, name: &str) -> Option<(i32, Expression)> {
        match name {
            "to_string" => Some((
                0,
                Expression::String(LoopString {
                    value: String::new(),
                }),
            )),
            &_ => None,
        }
    }
}
