use crate::object::function::{CompiledFunction, Function};
use crate::object::Object;
use crate::vm::VM;

pub fn run_function_stack(vm: &mut VM, constant: u32, param_count: u8) -> Option<String> {
    let func_obj = vm.bytecode.constants[constant as usize].clone();

    if let Object::CompiledFunction(func) = func_obj {
        let func = Object::Function(Function { func });

        vm.push(&func);
    }

    None
}
