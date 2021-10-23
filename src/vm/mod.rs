mod frame;
mod function;
mod suffix;
mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{read_uint32, read_uint8};
use crate::compiler::opcode::OpCode;
use crate::compiler::Bytecode;
use crate::lib::exception::vm::VMException;
use crate::lib::object::builtin::BUILTINS;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::null::Null;
use crate::lib::object::Object;
use crate::vm::frame::{build_frame, Frame};
use crate::vm::function::{run_function, run_function_stack};
use crate::vm::suffix::run_suffix_expression;
use std::collections::HashMap;
use std::rc::Rc;

pub struct VM {
    stack: Vec<Rc<Object>>,
    sp: u16,
    frames: Vec<Frame>,
    pub frame_index: usize,
    constants: Vec<Rc<Object>>,
    variables: HashMap<u32, Rc<Object>>,
}

const STACK_SIZE: usize = 2048;

pub struct VMState {
    variables: HashMap<u32, Rc<Object>>,
}

pub fn build_vm(bt: Bytecode, state: Option<&VMState>) -> VM {
    let mut stack = Vec::with_capacity(STACK_SIZE);

    for _ in 0..STACK_SIZE {
        stack.push(Rc::new(Object::Null(Null {})))
    }

    let default_frame = build_frame(
        Function {
            func: CompiledFunction {
                instructions: bt.instructions.clone(),
                num_locals: 0,
                num_parameters: 0,
            },
            free: vec![],
        },
        0,
    );

    if let Some(st) = state {
        return VM {
            stack,
            frames: vec![default_frame],
            frame_index: 0,
            sp: 0,
            constants: bt.constants,
            variables: st.variables.clone(),
        };
    }

    VM {
        stack,
        frames: vec![default_frame],
        frame_index: 0,
        sp: 0,
        constants: bt.constants,
        variables: HashMap::new(),
    }
}

impl VM {
    pub fn run(&mut self, attempt_jit: bool) -> Result<Rc<Object>, String> {
        while self.current_frame().ip < (self.current_frame().instructions().len()) as u32 {
            let ip = self.current_frame().ip;
            let _op = lookup_op(self.current_frame().instructions()[ip as usize]);

            if _op.is_none() {
                return Err(format!("OpCode not found: {}", ip));
            }

            let op = _op.unwrap();

            self.increment_ip(1);

            let err = match op {
                OpCode::Constant => {
                    let ip = self.current_frame().ip;
                    let idx = read_uint32(&self.current_frame().instructions()[ip as usize..]);
                    self.increment_ip(4);

                    self.push(Rc::clone(&self.constants[idx as usize]));

                    None
                }
                OpCode::Add => run_suffix_expression(self, "+"),
                OpCode::Modulo => run_suffix_expression(self, "%"),
                OpCode::Minus => run_suffix_expression(self, "-"),
                OpCode::Divide => run_suffix_expression(self, "/"),
                OpCode::Multiply => run_suffix_expression(self, "*"),
                OpCode::Pop => {
                    self.pop();
                    None
                }
                OpCode::Closure => None,
                OpCode::SetVar => {
                    let ip = self.current_frame().ip;
                    let idx = read_uint32(&self.current_frame().instructions()[ip as usize..]);
                    self.increment_ip(4);

                    let item = self.pop();
                    self.variables.insert(idx, item);
                    None
                }
                OpCode::GetVar => {
                    let ip = self.current_frame().ip;
                    let idx = read_uint32(&self.current_frame().instructions()[ip as usize..]);
                    self.increment_ip(4);

                    let variable = self.variables.get(&idx).unwrap().clone();

                    self.push(Rc::clone(&variable));

                    None
                }
                OpCode::Equals => run_suffix_expression(self, "=="),
                OpCode::NotEquals => run_suffix_expression(self, "!="),
                OpCode::GreaterThan => run_suffix_expression(self, ">"),
                OpCode::Jump => {
                    let ip = self.current_frame().ip;
                    let jump_to = read_uint32(&self.current_frame().instructions()[ip as usize..]);

                    self.set_ip(jump_to);

                    None
                }
                OpCode::JumpIfFalse => {
                    let condition = self.pop();

                    if !condition.is_truthy() {
                        let ip = self.current_frame().ip;

                        let jump_to =
                            read_uint32(&self.current_frame().instructions()[ip as usize..]);

                        self.set_ip(jump_to);
                    } else {
                        self.increment_ip(4);
                    }

                    None
                }
                OpCode::Return => {
                    let return_value = self.pop();
                    let base_pointer = self.pop_frame().base_pointer;

                    self.sp = (base_pointer - 1) as u16;

                    self.push(return_value)
                }
                OpCode::Function => {
                    let ip = self.current_frame().ip;

                    let ct = read_uint32(&self.current_frame().instructions()[ip as usize..]);
                    self.increment_ip(4);

                    let ip = self.current_frame().ip;

                    let parameters =
                        read_uint8(&self.current_frame().instructions()[ip as usize..]);
                    self.increment_ip(1);

                    run_function_stack(self, ct, parameters)
                }
                OpCode::Call => {
                    let ip = self.current_frame().ip;
                    let ins = self.current_frame().instructions();
                    let args = read_uint8(&ins[ip as usize..]);

                    self.increment_ip(1);

                    // TODO: Properly implement VM exceptions
                    let vm_exception = run_function(self, args, attempt_jit);

                    if let Some(exception) = vm_exception {
                        return match exception {
                            VMException::IncorrectArgumentCount(expected, got) => Err(format!(
                                "incorrect argument count. expected={}. got={}",
                                expected, got
                            )),
                            VMException::IncorrectType(message) => Err(message),
                            VMException::CannotParseInt(string) => {
                                Err(format!("unable to parse to int. got=\"{}\"", string))
                            }
                        };
                    }

                    None
                }
                OpCode::GetLocal => {
                    let ip = self.current_frame().ip;
                    let ins = self.current_frame().instructions();
                    let idx = read_uint8(&ins[ip as usize..]);
                    self.increment_ip(1);

                    let frame = self.current_frame();
                    let base_pointer = frame.base_pointer;

                    let local = Rc::clone(&self.stack[(base_pointer + (idx as i32)) as usize]);
                    self.push(local)
                }
                OpCode::GetFree => {
                    let ip = self.current_frame().ip;
                    let ins = self.current_frame().instructions();
                    let idx = read_uint8(&ins[ip as usize..]);
                    self.increment_ip(1);

                    let current = self.current_frame().func.clone();

                    let free = &current.free[idx as usize];

                    self.push(Rc::clone(free));

                    None
                }
                OpCode::GetBuiltin => {
                    let ip = self.current_frame().ip;

                    let ct =
                        read_uint8(&self.current_frame().instructions()[ip as usize..]) as usize;
                    self.increment_ip(1);

                    let builtin_function = Rc::new(BUILTINS[ct].builtin.clone());

                    self.push(builtin_function)
                }
                OpCode::CallExtension => {
                    let ip = self.current_frame().ip;

                    let method_id =
                        read_uint8(&self.current_frame().instructions()[ip as usize..]) as usize;
                    self.increment_ip(1);

                    let _parameters =
                        read_uint8(&self.current_frame().instructions()[(ip + 1) as usize..])
                            as usize;
                    self.increment_ip(1);

                    let perform_on = &*self.pop();

                    let method = perform_on.get_extension_method(method_id as i32);

                    let push = method.unwrap()(vec![]);

                    if push.is_err() {
                        return match push.err().unwrap() {
                            VMException::IncorrectArgumentCount(expected, got) => Err(format!(
                                "incorrect argument count. expected={}. got={}",
                                expected, got
                            )),
                            VMException::IncorrectType(message) => Err(message),
                            VMException::CannotParseInt(string) => {
                                Err(format!("unable to parse to int. got=\"{}\"", string))
                            }
                        };
                    }

                    let object = push.ok().unwrap();

                    self.push(Rc::from(object));

                    None
                }
            };

            if let Some(err) = err {
                return Err(err);
            }
        }

        Ok(Rc::clone(self.stack.first().unwrap()))
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
        self.frame_index += 1;
    }

    pub fn increment_ip(&mut self, increment: u32) {
        self.frames[self.frame_index].ip += increment;
    }
    pub fn set_ip(&mut self, ip: u32) {
        self.frames[self.frame_index].ip = ip;
    }

    pub fn pop_frame(&mut self) -> Frame {
        self.frame_index -= 1;

        self.frames.pop().unwrap()
    }

    pub fn current_frame(&mut self) -> &Frame {
        &self.frames[self.frame_index]
    }

    pub fn get_state(&self) -> VMState {
        VMState {
            variables: self.variables.clone(),
        }
    }

    pub fn push(&mut self, obj: Rc<Object>) -> Option<String> {
        if (self.sp + 1) as usize >= 2048 {
            panic!("stack overflow")
        }

        self.stack[self.sp as usize] = obj;

        self.sp += 1;

        None
    }

    pub fn pop(&mut self) -> Rc<Object> {
        let popped = self.stack[self.sp as usize - 1].to_owned();
        self.sp -= 1;

        popped
    }
}
