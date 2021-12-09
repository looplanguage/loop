use crate::lib::exception::vm::VMException;
use crate::lib::object::function::Function;
use crate::lib::object::Object;
use crate::vm::frame::build_frame;
use crate::vm::VM;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run_function(vm: &mut VM, num_args: u8, _attempt_jit: bool) -> Option<VMException> {
    let stack_item = &vm.stack[(vm.sp - 1 - (num_args as u16)) as usize].clone();
    let func_obj = &*(stack_item).borrow();

    match func_obj {
        Object::Function(func) => {
            let parameters = func.func.num_parameters;

            if parameters != num_args {
                return Some(VMException::IncorrectArgumentCount(
                    parameters as i32,
                    num_args as i32,
                ));
            }

            let num_locals = func.func.num_locals as usize;

            let base_pointer = vm.sp - (num_args as u16);

            let frame = build_frame(func.clone(), base_pointer as i32);

            vm.push_frame(frame);

            vm.sp = base_pointer + (num_locals as u16)
        }
        Object::Builtin(func) => {
            let mut args = Vec::with_capacity(num_args as usize);
            for _ in 0..num_args {
                let arg = vm.pop();

                args.push(arg.clone())
            }

            args.reverse();

            // Pop the function of the stack
            vm.pop();

            match func(args) {
                Ok(result) => {
                    vm.push(Rc::new(RefCell::from(result)));
                }
                Err(err) => {
                    return Some(err);
                }
            }
        }
        _ => {}
    }

    None
}

pub fn run_function_stack(vm: &mut VM, constant: u32, free_count: u8) -> Option<String> {
    let func_obj = vm.constants[constant as usize].clone();

    if let Object::CompiledFunction(func) = &*func_obj.as_ref().borrow() {
        let mut free = Vec::new();

        for _ in 0..free_count {
            free.push(Rc::clone(&vm.pop()));
        }
        free.reverse();

        let func = Object::Function(Function {
            func: func.clone(),
            free,
        });

        vm.push(Rc::new(RefCell::from(func)));
    }

    None
}
