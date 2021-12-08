mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{read_uint32, read_uint8};
use crate::compiler::opcode::OpCode;
use crate::lib::config::CONFIG;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::integer;
use crate::lib::object::null::Null;
use crate::lib::object::Object;
use crate::vm::VM;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValue, FunctionValue};
use inkwell::{AddressSpace, FloatPredicate};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type MainFunction = unsafe extern "C" fn() -> f64;

pub enum JitType {
    Number,
}

#[derive(Clone, Debug)]
pub enum StackItem<'ctx> {
    AnyValueEnum(AnyValueEnum<'ctx>),
    FunctionName(String),
}

pub struct CodeGen<'a, 'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub(crate) module: &'a Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) last_popped: Option<StackItem<'ctx>>,
}

// TODO: Document this quite a bit more, as this is a little complicated
impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn compile(
        &mut self,
        func: CompiledFunction,
        pointer: String,
        vm: &mut VM,
        arguments: Vec<JitType>,
    ) -> bool {
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
                pointer.starts_with("MAIN"),
            );

            if !ok {
                return false;
            }

            if CONFIG.debug_mode {
                println!("{}", self.module.print_to_string().to_string());
            }

            function.verify(true);

            self.fpm.run_on(&function);
        } else {
            let f64_type = self.context.f64_type();

            let fn_type = {
                let mut args: Vec<BasicMetadataTypeEnum> = Vec::new();

                for _ in arguments {
                    args.push(BasicMetadataTypeEnum::FloatType(f64_type));
                }

                f64_type.fn_type(&args, false)
            };

            let function = self.module.add_function(pointer.as_str(), fn_type, None);
            let basic_block = self.context.append_basic_block(function, "entry");

            self.builder.position_at_end(basic_block);

            let ok = self.compile_bytecode(
                func.instructions.clone(),
                function,
                vm,
                0,
                func.instructions.len() as u32,
                pointer.starts_with("MAIN"),
            );

            if !ok {
                return false;
            }

            function.verify(true);

            if CONFIG.debug_mode {
                println!("{}", self.module.print_to_string().to_string());
            }

            self.fpm.run_on(&function);

            if pointer.starts_with("MAIN") {
                self.fpm.run_on(&function);
            }
        }

        true
    }

    fn pop(&mut self, stack: &mut Vec<StackItem<'ctx>>) -> Option<StackItem<'ctx>> {
        self.last_popped = stack.pop();
        self.last_popped.clone()
    }

    fn push(&self, stack: &mut Vec<StackItem<'ctx>>, item: StackItem<'ctx>) {
        stack.push(item);
    }

    fn compile_bytecode(
        &mut self,
        code: Vec<u8>,
        function: FunctionValue<'ctx>,
        vm: &mut VM,
        from: u32,
        to: u32,
        is_main: bool,
    ) -> bool {
        let mut ip = from;
        let mut temp_stack: Vec<StackItem> = Vec::new();
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
                            self.push(
                                temp_stack.as_mut(),
                                StackItem::AnyValueEnum(
                                    self.context
                                        .f64_type()
                                        .const_float(int.value as f64)
                                        .as_any_value_enum(),
                                ),
                            );
                        }
                        Object::Null(_) => {
                            self.push(
                                temp_stack.as_mut(),
                                StackItem::AnyValueEnum(AnyValueEnum::from(
                                    self.context.f64_type().const_float(0.0),
                                )),
                            );
                        }
                        Object::Boolean(bool) => {
                            if bool.value {
                                self.push(
                                    temp_stack.as_mut(),
                                    StackItem::AnyValueEnum(
                                        self.context
                                            .bool_type()
                                            .const_int(1, false)
                                            .as_any_value_enum(),
                                    ),
                                );
                            } else {
                                self.push(
                                    temp_stack.as_mut(),
                                    StackItem::AnyValueEnum(
                                        self.context
                                            .bool_type()
                                            .const_int(0, false)
                                            .as_any_value_enum(),
                                    ),
                                );
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

                    let return_val = if let StackItem::AnyValueEnum(a) = return_val {
                        a
                    } else {
                        return false;
                    };

                    let return_val = match return_val {
                        AnyValueEnum::IntValue(int) => int.as_basic_value_enum(),
                        AnyValueEnum::FloatValue(float) => float.as_basic_value_enum(),
                        AnyValueEnum::PhiValue(phi) => phi.as_basic_value(),
                        _ => {
                            return false;
                        }
                    };

                    self.builder.build_return(Some(&return_val));
                }
                OpCode::GetLocal => {
                    let idx = read_uint8(&code[ip as usize..]);

                    ip += 1;

                    let param = function.get_nth_param(idx as u32);

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(param.unwrap().as_any_value_enum()),
                    );
                }
                OpCode::Add => {
                    let right = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    };

                    let left = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    };

                    let added = self.builder.build_float_add(
                        left.into_float_value(),
                        right.into_float_value(),
                        "add",
                    );

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(added.as_any_value_enum()),
                    );
                }
                OpCode::Multiply => {
                    let right = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    };

                    let left = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    };

                    let multiplied = self.builder.build_float_mul(
                        left.into_float_value(),
                        right.into_float_value(),
                        "add",
                    );

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(multiplied.as_any_value_enum()),
                    );
                }
                OpCode::Minus => {
                    let right = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    };

                    let left = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    };

                    let subtracted = self.builder.build_float_sub(
                        left.into_float_value(),
                        right.into_float_value(),
                        "add",
                    );

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(subtracted.as_any_value_enum()),
                    );
                }
                OpCode::Equals => {
                    let right = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    }
                    .into_float_value();

                    let left = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    }
                    .into_float_value();

                    let compared = self.builder.build_float_compare(
                        FloatPredicate::OEQ,
                        left,
                        right,
                        "compare",
                    );

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(compared.as_any_value_enum()),
                    );
                }
                OpCode::GreaterThan => {
                    let right = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    }
                    .into_float_value();

                    let left = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a
                        } else {
                            return false;
                        }
                    }
                    .into_float_value();

                    let compared = self.builder.build_float_compare(
                        FloatPredicate::OGT,
                        left,
                        right,
                        "compare",
                    );

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(compared.as_any_value_enum()),
                    );
                }
                OpCode::Function => {
                    let ct = read_uint32(&code[ip as usize..]);
                    ip += 4;

                    //let free_count = read_uint8(&code[ip as usize..]);

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
                            Object::CompiledFunction(cf) => cf.clone(),
                            _ => {
                                return false;
                            }
                        };

                        // This needs to be modified to support closures in JIT
                        let free = Vec::new();

                        /*
                        for _ in 0..free_count {
                            free.push(Rc::clone(&vm.pop()));
                        }*/

                        //free.reverse();

                        let func = Object::Function(Function { func, free });

                        vm.variables
                            .insert(idx, Rc::from(RefCell::from(func.clone())));
                    } else {
                        println!("WRONG OP TYPE!;");
                        return false;
                    }

                    //run_function_stack(vm, ct, parameters)
                }
                OpCode::GetVar => {
                    let idx = read_uint32(&code[ip as usize..]);
                    ip += 4;

                    if let Some(variable) = self.module.get_global(idx.to_string().as_str()) {
                        self.push(
                            temp_stack.as_mut(),
                            StackItem::AnyValueEnum(
                                self.builder
                                    .build_load(variable.as_pointer_value(), "load_var")
                                    .as_any_value_enum(),
                            ),
                        );
                    } else {
                        let variable = vm.variables.get(&idx).unwrap().clone();

                        match &*variable.as_ref().borrow() {
                            Object::Function(vf) => {
                                let f_name = format!("{:p}", &*variable.as_ref().borrow());

                                let f = self.module.get_function(&*f_name.clone());

                                if f.is_some() {
                                    self.push(
                                        temp_stack.as_mut(),
                                        StackItem::FunctionName(f_name.clone()),
                                    )
                                } else {
                                    let ptr = format!("{:p}", &*variable.as_ref().borrow());

                                    let f64_type = self.context.f64_type();

                                    let fn_type = {
                                        let mut args: Vec<BasicMetadataTypeEnum> = Vec::new();

                                        for _ in 0..vf.func.num_parameters {
                                            args.push(BasicMetadataTypeEnum::from(f64_type));
                                        }

                                        f64_type.fn_type(&args, false)
                                    };

                                    self.module
                                        .add_function(ptr.clone().as_str(), fn_type, None);

                                    compile_at_end.insert(ptr.clone(), vf.func.clone());

                                    self.push(
                                        temp_stack.as_mut(),
                                        StackItem::FunctionName(ptr.clone()),
                                    );
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

                    let value = self.pop(temp_stack.as_mut());

                    if value.is_none() {
                        return false;
                    }

                    let value = if let Some(StackItem::AnyValueEnum(a)) = value {
                        a
                    } else {
                        return false;
                    };

                    let value = match value {
                        AnyValueEnum::IntValue(int) => int.as_basic_value_enum(),
                        AnyValueEnum::FloatValue(float) => float.as_basic_value_enum(),
                        _ => {
                            return false;
                        }
                    };

                    if let Some(global) = self.module.get_global(idx.to_string().as_str()) {
                        let ptr = global.as_pointer_value();
                        self.builder.build_store(ptr, value);
                    } else {
                        let ptr = self.module.add_global(
                            self.context.f64_type(),
                            Some(AddressSpace::Generic),
                            idx.to_string().as_str(),
                        );

                        ptr.set_initializer(&value.into_float_value());
                    }
                }
                OpCode::Call => {
                    let args_amount = read_uint8(&code[ip as usize..]);

                    ip += 1;

                    let stack_item =
                        temp_stack[((temp_stack.len() as u8) - 1 - args_amount) as usize].clone();

                    let func = {
                        if let StackItem::FunctionName(name) = stack_item {
                            self.module.get_function(&*name).unwrap()
                        } else {
                            println!("No function: {:?}", stack_item);
                            return false;
                        }
                    };

                    let mut args: Vec<BasicMetadataValueEnum> = Vec::new();

                    for _ in 0..args_amount {
                        let arg = {
                            if let StackItem::AnyValueEnum(a) =
                                self.pop(temp_stack.as_mut()).unwrap()
                            {
                                a.into_float_value().as_basic_value_enum()
                            } else {
                                self.context
                                    .f64_type()
                                    .const_float(0.0)
                                    .as_basic_value_enum()
                            }
                        };

                        args.push(BasicMetadataValueEnum::from(arg));
                    }

                    let returns = self.builder.build_call(func, &args, "call");

                    // Final pop func of stack too
                    self.pop(temp_stack.as_mut());

                    self.push(
                        temp_stack.as_mut(),
                        StackItem::AnyValueEnum(returns.as_any_value_enum()),
                    );
                }
                OpCode::JumpIfFalse => {
                    let jump_to = read_uint32(&code[ip as usize..]);

                    ip += 4;

                    let cond = {
                        if let StackItem::AnyValueEnum(a) = self.pop(temp_stack.as_mut()).unwrap() {
                            a.into_int_value()
                        } else {
                            self.context.i64_type().const_int(0, false)
                        }
                    };

                    // branches
                    let then_b = self.context.append_basic_block(function, "then");
                    let else_b = self.context.append_basic_block(function, "else");
                    let cont_b = self.context.append_basic_block(function, "ifcont");

                    self.builder.build_conditional_branch(cond, then_b, else_b);

                    // then block
                    self.builder.position_at_end(then_b);

                    // do then block
                    let _done =
                        self.compile_bytecode(code.clone(), function, vm, ip, jump_to, false);

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
        }

        if is_main {
            if let Some(StackItem::AnyValueEnum(p)) = self.last_popped {
                self.builder.build_return(Some(&p.into_float_value()));
            } else {
                self.builder
                    .build_return(Some(&self.context.f64_type().const_float(0.0)));
            }
        }

        for (key, value) in &compile_at_end {
            let mut args = Vec::new();

            for _ in 0..value.num_parameters {
                args.push(JitType::Number);
            }

            self.compile(value.clone(), key.clone(), vm, args);
        }

        true
    }

    #[allow(dead_code)]
    pub fn run(&mut self, ptr: String, _params: Vec<Rc<RefCell<Object>>>) -> Object {
        if self.module.get_function(&*ptr).is_some() {
            let main_function: JitFunction<MainFunction> =
                unsafe { self.execution_engine.get_function(&*ptr).unwrap() };

            let returned = unsafe { main_function.call() };

            return Object::Integer(integer::Integer {
                value: returned as i64,
            });
        }

        Object::Null(Null {})
    }
}
