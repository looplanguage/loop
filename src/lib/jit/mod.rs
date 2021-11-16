use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use std::mem;
use std::rc::Rc;

#[allow(dead_code)]
pub struct JitFunction {
}

#[allow(dead_code)]
pub fn build_jit_function(instructions: Vec<u8>, constants: Vec<Rc<Object>>) -> JitFunction {
    JitFunction {
    }
}

// TODO: Document this quite a bit more, as this is a little complicated
impl JitFunction {
    #[allow(dead_code)]
    pub fn compile(&mut self) -> bool {
        true
    }

    #[allow(dead_code)]
    pub fn run(&self) {
        //Object::Integer(Integer { value: hello_fn() })
    }
}
