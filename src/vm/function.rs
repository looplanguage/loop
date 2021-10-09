use crate::compiler::instructions::print_instructions;
use crate::object::function::{CompiledFunction, Function};
use crate::object::Object;
use crate::vm::frame::build_frame;
use crate::vm::VM;

pub fn run_function(vm: &mut VM, func: Function, args: u8) -> Option<String> {
    let parameters = func.func.parameters.clone();

    if parameters.len() as u8 != args {
        return Some(format!(
            "incorrect argument count. expected={}. got={}",
            parameters.len(),
            args
        ));
    }

    let mut frame = build_frame(func, (vm.sp as u16) as i32);

    print_instructions((frame.instructions().clone()).to_owned());

    let base_pointer = frame.base_pointer.clone();

    for parameter in parameters {
        if vm.variables.contains_key(&parameter) {
            vm.variables.remove(&parameter);
        }

        let value = vm.pop();
        vm.variables.insert(parameter, value);
    }

    vm.push_frame(frame);

    vm.sp = base_pointer as u16;

    None
}

pub fn run_function_stack(vm: &mut VM, constant: u32, param_count: u8) -> Option<String> {
    let func_obj = vm.constants[constant as usize].clone();

    if let Object::CompiledFunction(func) = func_obj {
        let func = Object::Function(Function { func });

        vm.push(&func);
    }

    None
}
