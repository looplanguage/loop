mod function;
mod suffix;
mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{read_uint32, read_uint8};
use crate::compiler::opcode::OpCode;
use crate::compiler::Bytecode;
use crate::object::integer::Integer;
use crate::object::Object;
use crate::vm::function::run_function_stack;
use crate::vm::suffix::run_suffix_expression;
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Object>,
    ip: u32,
    sp: u16,
    bytecode: Bytecode,
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
            ip: 0,
            sp: 0,
            bytecode: bt,
            last_popped: None,
            variables: st.variables.clone(),
        };
    }

    VM {
        stack: vec![],
        ip: 0,
        sp: 0,
        bytecode: bt,
        last_popped: None,
        variables: HashMap::new(),
    }
}

impl VM {
    pub fn run(&mut self) -> Option<String> {
        while self.ip < (self.bytecode.instructions.len() as u32) {
            let _op = lookup_op(self.bytecode.instructions[self.ip as usize]);

            _op.as_ref()?;

            let op = _op.unwrap();

            self.ip += 1;
            let err = match op {
                OpCode::Constant => {
                    let idx =
                        read_uint32(self.bytecode.instructions[self.ip as usize..].to_owned());
                    self.ip += 4;

                    self.push(&self.bytecode.constants[idx as usize].clone());

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
                    let idx =
                        read_uint32(self.bytecode.instructions[self.ip as usize..].to_owned());
                    self.ip += 4;

                    let item = self.pop();
                    self.variables.insert(idx, item);
                    None
                }
                OpCode::GetVar => {
                    let idx =
                        read_uint32(self.bytecode.instructions[self.ip as usize..].to_owned());
                    self.ip += 4;

                    let variable = self.variables.get(&idx).unwrap().clone();

                    self.push(&variable);

                    None
                }
                OpCode::Equals => run_suffix_expression(self, "=="),
                OpCode::NotEquals => run_suffix_expression(self, "!="),
                OpCode::GreaterThan => run_suffix_expression(self, ">"),
                OpCode::Jump => {
                    let jump_to =
                        read_uint32(self.bytecode.instructions[self.ip as usize..].to_owned());

                    self.ip = jump_to;

                    None
                }
                OpCode::JumpIfFalse => {
                    let condition = self.pop();

                    if let Object::Boolean(dont_jump) = condition {
                        if !dont_jump.value {
                            let jump_to = read_uint32(
                                self.bytecode.instructions[self.ip as usize..].to_owned(),
                            );

                            self.ip = jump_to;
                        } else {
                            self.ip += 4;
                        };

                        None
                    } else {
                        Some(format!(
                            "unable to jump. got=\"{:?}\". expected=\"Boolean\"",
                            condition
                        ))
                    }
                }
                OpCode::Return => None,
                OpCode::Function => {
                    let ct = read_uint32(self.bytecode.instructions[self.ip as usize..].to_owned());
                    self.ip += 4;

                    let parameters =
                        read_uint8(self.bytecode.instructions[self.ip as usize..].to_owned());
                    self.ip += 1;

                    run_function_stack(self, ct, parameters)
                }
            };

            if err.is_some() {
                return err;
            }
        }

        None
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
