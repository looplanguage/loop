mod suffix;
mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::read_uint32;
use crate::compiler::opcode::OpCode;
use crate::compiler::Bytecode;
use crate::object::integer::Integer;
use crate::object::Object;
use crate::vm::suffix::run_suffix_expression;
use std::collections::HashMap;

pub struct VM {
    stack: [Object; 2048],
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
            stack: [Object::Integer(Integer { value: 0 }); 2048],
            ip: 0,
            sp: 0,
            bytecode: bt,
            last_popped: None,
            variables: st.variables.clone(),
        };
    }

    VM {
        stack: [Object::Integer(Integer { value: 0 }); 2048],
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

                    self.push(self.bytecode.constants[idx as usize]);

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

                    self.push(*self.variables.get(&idx).unwrap());
                    None
                }
                OpCode::Equals => run_suffix_expression(self, "=="),
                OpCode::NotEquals => run_suffix_expression(self, "!="),
                OpCode::GreaterThan => run_suffix_expression(self, ">"),
                OpCode::Jump => {
                    self.ip += 4;
                    None
                }
                OpCode::JumpIfFalse => {
                    let condition = self.pop();

                    if let Object::Boolean(dont_jump) = condition {
                        return if !dont_jump.value {
                            println!("Jumping...");
                            let jump_to = read_uint32(
                                self.bytecode.instructions[self.ip as usize..].to_owned(),
                            );

                            self.ip = jump_to;
                            None
                        } else {
                            self.ip += 4;
                            None
                        };
                    }

                    Some(format!(
                        "unable to jump. got=\"{:?}\". expected=\"Boolean\"",
                        condition
                    ))
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

    pub fn push(&mut self, obj: Object) -> Option<String> {
        if (self.sp + 1) as usize >= self.stack.len() {
            panic!("stack overflow")
        }

        self.stack[self.sp as usize] = obj;

        self.sp += 1;

        None
    }

    pub fn pop(&mut self) -> Object {
        let popped = self.stack[(self.sp - 1) as usize];

        if self.sp == 0 {
            panic!("can not pop nothing of the stack");
        } else {
            self.sp -= 1;
            self.last_popped = Some(popped);

            popped
        }
    }
}
