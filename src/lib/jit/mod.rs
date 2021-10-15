use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::read_uint32;
use crate::compiler::opcode::OpCode;
use crate::lib::object;
use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use crate::lib::object::Object::Null;
use dynasmrt::{dynasm, AssemblyOffset, DynasmApi, DynasmLabelApi, ExecutableBuffer};
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;
use std::{io, mem, slice};

pub struct JitFunction {
    pub(crate) ip: i32,
    pub(crate) instructions: Vec<u8>,
    pub(crate) pointer: Option<AssemblyOffset>,
    pub(crate) buffer: Option<ExecutableBuffer>,
}

impl JitFunction {
    pub fn compile(&mut self, constants: Vec<Rc<Object>>) -> bool {
        let mut ops = dynasmrt::x64::Assembler::new().unwrap();

        dynasm!(ops
            ; .arch x64
        );

        let offset = ops.offset();

        let mut available_addresses: Vec<&str> = vec!["rcx", "rbx", "rax"];

        while self.ip < (self.instructions.len()) as i32 {
            let ip = self.ip;
            let _op = lookup_op(self.instructions[ip as usize]);

            let op = _op.unwrap();

            self.ip += 1;

            let err = match op {
                OpCode::Constant => {
                    let ip = self.ip;
                    let idx = read_uint32(self.instructions[ip as usize..].to_owned());
                    self.ip += 4;

                    let number = &constants[idx as usize];

                    if let Object::Integer(number) = number.clone().deref() {
                        let adr = available_addresses.pop().unwrap();

                        println!("{}", adr);
                        match adr {
                            "rax" => {
                                dynasm!(ops
                                    ; mov rax, number.value as _
                                );
                            }
                            "rbx" => {
                                dynasm!(ops
                                    ; mov rbx, number.value as _
                                );
                            }
                            "rcx" => {
                                dynasm!(ops
                                    ; mov rcx, number.value as _
                                );
                            }
                            _ => {}
                        }
                    }
                }
                OpCode::Add => {
                    dynasm!(ops
                        ; add rax, rbx
                    );

                    available_addresses.push("rbx");
                }
                OpCode::Multiply => {
                    dynasm!(ops
                        ; mul rbx
                    );

                    available_addresses.push("rbx");
                }
                _ => {
                    return false;
                }
            };
        }

        dynasm!(ops
            ; ret
        );

        let buf = ops.finalize().unwrap();

        let pointer = offset;

        self.pointer = Option::from(pointer);
        self.buffer = Option::from(buf);

        true
    }

    pub fn run(&self) -> Object {
        let buf = self.buffer.as_ref().unwrap();

        let hello_fn: extern "win64" fn() -> i64 =
            unsafe { mem::transmute(buf.ptr(self.pointer.unwrap())) };

        Object::Integer(Integer { value: hello_fn() })
    }
}
