use crate::ast::instructions::Node;
use crate::types::Type;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<Type>,
    pub free: Vec<Type>,
    pub body: Vec<Node>,
    pub unique_identifier: i32,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".FUNCTION {:?} {} ARGUMENTS {{{:?}}} FREE {{{:?}}} THEN {{{:?}}}",
            self.return_type, self.name, self.parameters, self.free, self.body
        )
    }
}

#[derive(PartialEq, Clone)]
pub struct Call {
    pub call: Node,
    pub arguments: Vec<Node>,
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ".CALL {} {{{:?}}}", self.call, self.arguments)
    }
}

#[derive(PartialEq, Clone)]
pub struct LibCall {
    pub namespace: String,
    pub arguments: Vec<Node>,
}

impl Display for LibCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ".LIBCALL {} {{{:?}}}", self.namespace, self.arguments)
    }
}