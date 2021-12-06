mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{read_uint32, read_uint8};
use crate::compiler::opcode::OpCode;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::integer;
use crate::lib::object::null::Null;
use crate::lib::object::Object;
use crate::vm::VM;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{AnyValue, AnyValueEnum, BasicValue, CallableValue, FunctionValue, PointerValue};
use crate::vm::function::run_function_stack;
use inkwell::FloatPredicate;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use inkwell::passes::PassManager;

// Stubs

pub enum Stub<'ctx> {
    F64RF64(JitFunction<'ctx, StubF64RF64>),
}

type StubF64RF64 = unsafe extern "C" fn(f64) -> f64;

#[allow(dead_code)]
pub struct CodeGen<'a, 'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub(crate) module: &'a Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) compiled_functions: HashMap<String, Stub<'ctx>>,
    pub(crate) parameters: Vec<String>,
    pub(crate) jit_variables: HashMap<String, PointerValue<'ctx>>,
    pub(crate) last_popped: Option<AnyValueEnum<'ctx>>,
}

// TODO: Document this quite a bit more, as this is a little complicated
impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn get_function(&self, pointer: String) -> Option<&Stub<'ctx>> {
        self.compiled_functions.get(&*pointer)
    }

    pub fn compile(&mut self, func: CompiledFunction, pointer: String, vm: &mut VM) -> bool {
        let exists = self.module.get_function(pointer.clone().as_str());

        if let Some(function) = exists {
            let basic_block = self.context.append_basic_block(function, "entry");

            self.builder.position_at_end(basic_block);

            let ok = self.compile_bytecode(
                func.instructions.clone(),
                function,
                vm,
                0,
                func.instructions.len() as u32,
                false
            );

            if !ok {
                return false;
            }

            self.compiled_functions.insert(pointer.clone(), unsafe {
                Stub::F64RF64(
                    self.execution_engine
                        .get_function(pointer.as_str())
                        .ok()
                        .unwrap(),
                )
            });

            function.verify(true);

            self.fpm.run_on(&function);
        } else {
            let f64_type = self.context.f64_type();
            let fn_type = f64_type.fn_type(&[f64_type.into()], false);
            let function = self
                .module
                .add_function(pointer.clone().as_str(), fn_type, None);
            let basic_block = self.context.append_basic_block(function, "entry");

            self.builder.position_at_end(basic_block);

            let ok = self.compile_bytecode(
                func.instructions.clone(),
                function,
                vm,
                0,
                func.instructions.len() as u32,
                (pointer == "MAIN")
            );

            if !ok {
                return false;
            }

            function.verify(true);

            self.fpm.run_on(&function);

            if pointer == "MAIN" {
                self.compiled_functions.insert(pointer.clone(), unsafe {
                    Stub::F64RF64(
                        self.execution_engine
                            .get_function(pointer.as_str())
                            .ok()
                            .unwrap(),
                    )
                });
            }
        }

        true
    }

    fn pop(&mut self, stack: &mut Vec<AnyValueEnum<'ctx>>) -> Option<AnyValueEnum<'ctx>> {
        self.last_popped = stack.pop();
        self.last_popped
    }

    fn push(&self, stack: &mut Vec<AnyValueEnum<'ctx>>, item: AnyValueEnum<'ctx>) {
        stack.push(item);
    }

    fn compile_bytecode(
        &mut self,
        code: Vec<u8>,
        function: FunctionValue<'ctx>,
        vm: &mut VM,
        from: u32,
        to: u32,
        is_main: bool
    ) -> bool {
        let mut ip = from;
        let mut temp_stack: Vec<AnyValueEnum> = Vec::new();
        let mut compile_at_end: HashMap<String, CompiledFunction> = HashMap::new();

        while ip < (code.len() as u32) {
            let _op = lookup_op(code[ip as usize]);

            if _op.is_none() {
                return false;
            }

            let op = _op.unwrap();

            if ip == to && to != code.len() as u32 {
                return true;
            }

            ip += 1;

            match op {
                OpCode::Constant => {
                    let idx = read_uint32(&code[ip as usize..]);
                    ip += 4;

                    let cst = vm.constants[idx as usize].clone();

                    match &*cst.as_ref().borrow() {
                        Object::Integer(int) => {
                            self.push(temp_stack.as_mut(), self.context
                                .f64_type()
                                .const_float(int.value as f64)
                                .as_any_value_enum());
                        }
                        Object::Null(_) => {
                            self.push(temp_stack.as_mut(), AnyValueEnum::from(self.context.f64_type().const_float(0.0)));
                        }
                        Object::Boolean(bool) => {
                            if bool.value {
                                self.push(temp_stack.as_mut(), self.context
                                    .bool_type()
                                    .const_int(1, false)
                                    .as_any_value_enum());
                            } else {
                                self.push(temp_stack.as_mut(), self.context
                                    .bool_type()
                                    .const_int(0, false)
                                    .as_any_value_enum());
                            }
                        }
                        _ => {
                            println!("UNKNOWN: {:?}", cst);
                            return false;
                        }
                    };
                }
                OpCode::Return => {
                    let return_val = self.pop(temp_stack.as_mut()).unwrap();

                    let return_val = match return_val {
                        AnyValueEnum::IntValue(int) => int.as_basic_value_enum(),
                        AnyValueEnum::FloatValue(float) => float.as_basic_value_enum(),
                        AnyValueEnum::PhiValue(phi) => phi.as_basic_value(),
                        _ => {
                            return false;
                        }
                    };

                    // Causes STATUS_ACCESS_VIOLATION when inside an "if-block"
                    self.builder.build_return(Some(&return_val));
                }
                OpCode::GetLocal => {
                    let idx = read_uint8(&code[ip as usize..]);

                    ip += 1;

                    let param = function.get_nth_param(idx as u32);

                    self.push(temp_stack.as_mut(), param.unwrap().as_any_value_enum());
                }
                OpCode::Add => {
                    let right = self.pop(temp_stack.as_mut()).unwrap();
                    let left = self.pop(temp_stack.as_mut()).unwrap();

                    let added = self.builder.build_float_add(
                        left.into_float_value(),
                        right.into_float_value(),
                        "add",
                    );

                    self.push(temp_stack.as_mut(), added.as_any_value_enum());
                }
                OpCode::Multiply => {
                    let right = self.pop(temp_stack.as_mut()).unwrap();
                    let left = self.pop(temp_stack.as_mut()).unwrap();

                    let multiplied = self.builder.build_float_mul(
                        left.into_float_value(),
                        right.into_float_value(),
                        "add",
                    );

                    self.push(temp_stack.as_mut(), multiplied.as_any_value_enum());
                }
                OpCode::Minus => {
                    let right = self.pop(temp_stack.as_mut()).unwrap();
                    let left = self.pop(temp_stack.as_mut()).unwrap();

                    let subtracted = self.builder.build_float_sub(
                        left.into_float_value(),
                        right.into_float_value(),
                        "add",
                    );

                    self.push(temp_stack.as_mut(), subtracted.as_any_value_enum());
                }
                OpCode::Equals => {
                    let right = self.pop(temp_stack.as_mut()).unwrap().into_float_value();
                    let left = self.pop(temp_stack.as_mut()).unwrap().into_float_value();

                    let compared = self.builder.build_float_compare(
                        FloatPredicate::OEQ,
                        left,
                        right,
                        "compare",
                    );

                    self.push(temp_stack.as_mut(), compared.as_any_value_enum());
                }
                OpCode::GreaterThan => {
                    let right = self.pop(temp_stack.as_mut()).unwrap().into_float_value();
                    let left = self.pop(temp_stack.as_mut()).unwrap().into_float_value();

                    let compared = self.builder.build_float_compare(
                        FloatPredicate::OGT,
                        left,
                        right,
                        "compare",
                    );

                    self.push(temp_stack.as_mut(), compared.as_any_value_enum());
                }
                OpCode::Function => {
                    let ct = read_uint32(&code[ip as usize..]);
                    ip += 4;

                    let free_count =
                        read_uint8(&code[ip as usize..]);

                    ip += 1;

                    // For now we are only handling named functions (not lambda/anonymous functions)
                    let _op = lookup_op(code[ip as usize]);

                    if _op.is_none() {
                        println!("NO OP TYPE!;");
                        return false;
                    }

                    let op = _op.unwrap();
                    ip += 1;

                    if let OpCode::SetVar = op {
                        let idx = read_uint32(&code[ip as usize..]);
                        ip += 4;


                        let func = match &*vm.constants[ct as usize].clone().as_ref().borrow() {
                            Object::CompiledFunction(cf) => { cf.clone() }
                            _ => {
                                return false;
                            }
                        };

                        // This needs to be modified to support closures in JIT
                        let mut free = Vec::new();

                        /*
                        for _ in 0..free_count {
                            free.push(Rc::clone(&vm.pop()));
                        }*/

                        //free.reverse();

                        let func = Object::Function(Function {
                            func,
                            free,
                        });

                        vm.variables.insert(idx, Rc::from(RefCell::from(func.clone())));
                    } else {
                        println!("WRONG OP TYPE!;");
                        return false;
                    }

                    //run_function_stack(vm, ct, parameters)
                }
                OpCode::GetVar => {
                    let idx = read_uint32(&code[ip as usize..]);
                    ip += 4;

                    if let Some(variable) = self.jit_variables.get(idx.to_string().as_str()) {
                        self.push(temp_stack.as_mut(), self.builder.build_load(*variable, "load_var").as_any_value_enum());
                    } else {
                        let variable = vm.variables.get(&idx).unwrap().clone();

                        match &*variable.as_ref().borrow() {
                            Object::Function(vf) => {
                                let f = self
                                    .module
                                    .get_function(&*format!("{:p}", &*variable.as_ref().borrow()));

                                if let Some(f) = f {
                                    self.push(temp_stack.as_mut(), f.as_any_value_enum());
                                } else {
                                    let ptr = format!("{:p}", &*variable.as_ref().borrow());

                                    let f64_type = self.context.f64_type();
                                    let fn_type = f64_type.fn_type(&[f64_type.into()], false);
                                    let function = self
                                        .module
                                        .add_function(ptr.clone().as_str(), fn_type, None);

                                    compile_at_end.insert(ptr.clone(), vf.func.clone());

                                    self.push(temp_stack.as_mut(), function.as_any_value_enum());

                                }
                            }
                            _ => {
                                println!(":( {:?}", variable);
                                return false;
                            }
                        };
                    };
                }
                OpCode::SetVar => {
                    let idx = read_uint32(&code[ip as usize..]);
                    ip += 4;

                    self.jit_variables.remove(idx.to_string().as_str());

                    let value = self.pop(temp_stack.as_mut());

                    if value.is_none() {
                        return false;
                    }

                   let value = match value.unwrap() {
                        AnyValueEnum::IntValue(int) => int.as_basic_value_enum(),
                        AnyValueEnum::FloatValue(float) => float.as_basic_value_enum(),
                        _ => { return false; }
                    };

                    let alloca = self.create_entry_block_alloca(idx.to_string().as_str(), function);
                    self.builder.build_store(alloca, value);

                    self.jit_variables.insert(idx.to_string(), alloca);
                }
                OpCode::Call => {
                    let args = read_uint8(&code[ip as usize..]);

                    ip += 1;

                    let func = CallableValue::try_from(
                        temp_stack[((temp_stack.len() as u8) - 1 - args) as usize]
                            .into_pointer_value(),
                    )
                    .unwrap();

                    let param = self.pop(temp_stack.as_mut()).unwrap().into_float_value();

                    let returns = self.builder.build_call(func, &[param.into()], "call");

                    // Final pop func of stack too
                    self.pop(temp_stack.as_mut());

                    self.push(temp_stack.as_mut(), returns.as_any_value_enum());
                }
                OpCode::JumpIfFalse => {
                    let jump_to = read_uint32(&code[ip as usize..]);

                    ip += 4;

                    let cond = self.pop(temp_stack.as_mut()).unwrap().into_int_value();

                    // branches
                    let then_b = self.context.append_basic_block(function, "then");
                    let else_b = self.context.append_basic_block(function, "else");
                    let cont_b = self.context.append_basic_block(function, "ifcont");

                    self.builder.build_conditional_branch(cond, then_b, else_b);

                    // then block
                    self.builder.position_at_end(then_b);

                    // do then block
                    let _done = self.compile_bytecode(code.clone(), function, vm, ip, jump_to, false);

                    //self.builder.build_unconditional_branch(cont_b);

                    // else
                    self.builder.position_at_end(else_b);
                    self.builder.build_unconditional_branch(cont_b);

                    ip = jump_to;
                    //println!("Done: {}", done);

                    self.builder.position_at_end(cont_b);
                }
                OpCode::Jump => ip += 4,
                OpCode::Pop => {
                    self.pop(temp_stack.as_mut());
                }
                _ => {
                    println!("Unknown instruction: {:?}", op);
                    return false;
                }
            }
        };

        if is_main {
            if let Some(p) = self.last_popped {
                self.builder.build_return(Some(&self.last_popped.unwrap().into_float_value()));
            } else {
                self.builder.build_return(Some(&self.context.f64_type().const_float(0.0)));
            }
        }

        for (key, value) in &compile_at_end {
            self.compile(value.clone(), key.clone(), vm);
        }

        true
    }

    fn create_entry_block_alloca(&self, name: &str, function: FunctionValue<'ctx>) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry)
        }

        builder.build_alloca(self.context.f64_type(), name)
    }

    #[allow(dead_code)]
    pub fn run(&mut self, ptr: String, _params: Vec<Rc<RefCell<Object>>>) -> Object {
        if let Some(compiled) = self.get_function(ptr) {
            let mut _compiled_down_params: Vec<f64> = Vec::new();

            for _param in _params {
                if let Object::Integer(integer) = &*_param.as_ref().borrow() {
                    _compiled_down_params.push(integer.value as f64);
                }
            }

            let returned = match compiled {
                Stub::F64RF64(func) => unsafe { func.call(_compiled_down_params[0]) },
            };

            return Object::Integer(integer::Integer {
                value: returned as i64,
            });
        }

        Object::Null(Null {})
    }
}
