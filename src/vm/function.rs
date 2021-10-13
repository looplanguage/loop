use crate::lib::object::function::Function;
use crate::lib::object::Object;
use crate::vm::frame::build_frame;
use crate::vm::VM;
use std::rc::Rc;

pub fn run_function(vm: &mut VM, args: u8) -> Option<String> {
    let func_obj = (*vm.stack[(vm.sp - 1 - (args as u16)) as usize]).clone();

    if let Object::Function(func) = func_obj {
        let parameters = func.func.num_parameters;

        if parameters != args {
            return Some(format!(
                "incorrect argument count. expected={}. got={}",
                parameters, args
            ));
        }

        let num_locals = func.func.num_locals as usize;
        let base_pointer = vm.sp - (args as u16);

        let frame = build_frame(func, base_pointer as i32);

        vm.push_frame(frame);

        vm.sp = base_pointer + (num_locals as u16)
    }

    None
}

pub fn run_function_stack(vm: &mut VM, constant: u32, free_count: u8) -> Option<String> {
    let func_obj = vm.constants[constant as usize].clone();

    if let Object::CompiledFunction(func) = func_obj {
        let mut free = Vec::new();

        for _ in 0..free_count {
            free.push(Rc::clone(&vm.pop()));
        }
        free.reverse();

        let func = Object::Function(Function { func, free });

        vm.push(Rc::new(func));
    }

    None
}
