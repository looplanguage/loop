use crate::compiler::instructions::Instructions;
use crate::object::{Object, ObjectTrait};

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Vec<u8>,
    pub parameters: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub func: CompiledFunction,
    pub free: Vec<Object>,
}

impl ObjectTrait for CompiledFunction {
    fn inspect(&self) -> String {
        format!("CompiledFunction[{:p}]", self)
    }
}

impl ObjectTrait for Function {
    fn inspect(&self) -> String {
        format!("Function[{}]", self.func.parameters.len())
    }
}
