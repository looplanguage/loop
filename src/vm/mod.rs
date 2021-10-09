mod frame;
mod function;
mod suffix;
mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{read_uint32, read_uint8};
use crate::compiler::opcode::OpCode;
use crate::compiler::Bytecode;
use crate::object::function::{CompiledFunction, Function};
use crate::object::integer::Integer;
use crate::object::Object;
use crate::vm::frame::{build_frame, Frame};
use crate::vm::function::{run_function, run_function_stack};
use crate::vm::suffix::run_suffix_expression;
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Object>,
    sp: u16,
    frames: Vec<Frame>,
    pub frame_index: i32,
    constants: Vec<Object>,
    pub last_popped: Option<Object>,
    variables: HashMap<u32, Object>,
}

pub struct VMState {
    variables: HashMap<u32, Object>,
}

pub fn build_vm(bt: Bytecode, state: Option<&VMState>) -> VM {
    if let Some(st) = state {
        return VM {
            stack: vec![],
            frames: vec![build_frame(
                Function {
                    func: CompiledFunction {
                        instructions: bt.instructions.clone(),
                        parameters: vec![],
                    },
                },
                0,
            )],
            frame_index: 0,
            sp: 0,
            constants: bt.constants,
            last_popped: None,
            variables: st.variables.clone(),
        };
    }

    VM {
        stack: vec![],
        frames: vec![build_frame(
            Function {
                func: CompiledFunction {
                    instructions: bt.instructions.clone(),
                    parameters: vec![],
                },
            },
            0,
        )],
        frame_index: 0,
        sp: 0,
        constants: bt.constants,
        last_popped: None,
        variables: HashMap::new(),
    }
}

impl VM {
    pub fn run(&mut self) -> Option<String> {
        while self.current_frame().ip < (self.current_frame().instructions().len()) as u32 {
            let ip = self.current_frame().ip;
            let _op = lookup_op(self.current_frame().instructions()[ip as usize]);

            _op.as_ref()?;

            let op = _op.unwrap();

            self.current_frame().ip += 1;
            let err = match op {
                OpCode::Constant => {
                    let ip = self.current_frame().ip;
                    let idx =
                        read_uint32(self.current_frame().instructions()[ip as usize..].to_owned());
                    self.current_frame().ip += 4;

                    self.push(&self.constants[idx as usize].clone());

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
                    let idx =
                        read_uint32(self.current_frame().instructions()[ip as usize..].to_owned());
                    self.current_frame().ip += 4;

                    let item = self.pop();
                    self.variables.insert(idx, item);
                    None
                }
                OpCode::GetVar => {
                    let ip = self.current_frame().ip;
                    let idx =
                        read_uint32(self.current_frame().instructions()[ip as usize..].to_owned());
                    self.current_frame().ip += 4;

                    let variable = self.variables.get(&idx).unwrap().clone();

                    self.push(&variable);

                    None
                }
                OpCode::Equals => run_suffix_expression(self, "=="),
                OpCode::NotEquals => run_suffix_expression(self, "!="),
                OpCode::GreaterThan => run_suffix_expression(self, ">"),
                OpCode::Jump => {
                    let ip = self.current_frame().ip;
                    let jump_to =
                        read_uint32(self.current_frame().instructions()[ip as usize..].to_owned());

                    self.current_frame().ip = jump_to;

                    None
                }
                OpCode::JumpIfFalse => {
                    let condition = self.pop();

                    if let Object::Boolean(dont_jump) = condition {
                        if !dont_jump.value {
                            let ip = self.current_frame().ip;

                            let jump_to = read_uint32(
                                self.current_frame().instructions()[ip as usize..].to_owned(),
                            );

                            self.current_frame().ip = jump_to;
                        } else {
                            self.current_frame().ip += 4;
                        };

                        None
                    } else {
                        Some(format!(
                            "unable to jump. got=\"{:?}\". expected=\"Boolean\"",
                            condition
                        ))
                    }
                }
                OpCode::Return => {
                    let return_value = self.pop();
                    let frame = self.pop_frame();

                    self.sp = (frame.base_pointer) as u16;

                    self.push(&return_value)
                }
                OpCode::Function => {
                    let ip = self.current_frame().ip;

                    let ct =
                        read_uint32(self.current_frame().instructions()[ip as usize..].to_owned());
                    self.current_frame().ip += 4;

                    let ip = self.current_frame().ip;

                    let parameters =
                        read_uint8(self.current_frame().instructions()[ip as usize..].to_owned());
                    self.current_frame().ip += 1;

                    run_function_stack(self, ct, parameters)
                }
                OpCode::Call => {
                    let func = self.pop();
                    let ip = self.current_frame().ip;
                    let ins = self.current_frame().instructions();
                    let args = read_uint8(ins[ip as usize..].to_owned());

                    self.current_frame().ip += 1;

                    let mut err = None;
                    if let Object::Function(fun) = func {
                        err = run_function(self, fun, args);
                    }

                    err
                }
            };

            if err.is_some() {
                return err;
            }
        }

        None
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
        self.frame_index += 1;
    }

    pub fn pop_frame(&mut self) -> Frame {
        self.frame_index -= 1;

        self.frames.pop().unwrap()
    }

    pub fn current_frame(&mut self) -> &mut Frame {
        self.frames[self.frame_index as usize].borrow_mut()
    }

    pub fn get_state(&self) -> VMState {
        VMState {
            variables: self.variables.clone(),
        }
    }

    pub fn push(&mut self, obj: &Object) -> Option<String> {
        if (self.sp + 1) as usize >= 2048 {
            panic!("stack overflow")
        }

        self.stack.push(obj.clone());

        self.sp += 1;

        None
    }

    pub fn pop(&mut self) -> Object {
        if self.sp == 0 {
            panic!("can not pop nothing of the stack");
        } else {
            let popped = self.stack.pop().unwrap();

            self.sp -= 1;
            self.last_popped = Some(popped.clone());

            popped
        }
    }
}
