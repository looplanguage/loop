use crate::lib::exception::vm::VMException;
use crate::lib::object::builtin::EvalResult;
use crate::lib::object::function::Function;
use crate::lib::object::Object;
use crate::vm::frame::build_frame;
use crate::vm::VM;
use std::borrow::Borrow;
use std::rc::Rc;

pub fn run_function(vm: &mut VM, num_args: u8, _attempt_jit: bool) -> Option<VMException> {
    let func_obj = (*vm.stack[(vm.sp - 1 - (num_args as u16)) as usize]).clone();
    match func_obj {
        Object::Function(func) => {
            // Attempt to JIT the function, otherwise fall back to interpreted execution.
            // TODO: Re-enable when more thoroughly tested and developed
            /*
            if attempt_jit {
                let mut jit_func =
                    build_jit_function(func.func.instructions.clone(), vm.constants.clone());

                let compile_success = { jit_func.compile() };

                if compile_success {
                    vm.pop();
                    vm.push(Rc::from(jit_func.run()));
                    return None;
                }
            }
             */

            let parameters = func.func.num_parameters;

            if parameters != num_args {
                return Some(VMException::IncorrectArgumentCount(
                    parameters as i32,
                    num_args as i32,
                ));
            }

            let num_locals = func.func.num_locals as usize;
            let base_pointer = vm.sp - (num_args as u16);

            let frame = build_frame(func, base_pointer as i32);

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

            let _function = vm.pop();

            match func(args) {
                Ok(result) => {
                    vm.push(Rc::new(result));
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

    if let Object::CompiledFunction(func) = func_obj.borrow() {
        let mut free = Vec::new();

        for _ in 0..free_count {
            free.push(Rc::clone(&vm.pop()));
        }
        free.reverse();

        let func = Object::Function(Function {
            func: func.clone(),
            free,
        });

        vm.push(Rc::new(func));
    }

    None
}
