use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use dynasmrt::{dynasm, AssemblyOffset, ExecutableBuffer, DynasmApi};
use std::rc::Rc;
use std::mem;

#[allow(dead_code)]
pub struct JitFunction {
    pub(crate) ip: i32,
    pub(crate) instructions: Vec<u8>,
    pub(crate) pointer: Option<AssemblyOffset>,
    pub(crate) buffer: Option<ExecutableBuffer>,
    pub(crate) constants: Vec<Rc<Object>>,
    last_used_adr: String,
}

#[allow(dead_code)]
pub fn build_jit_function(instructions: Vec<u8>, constants: Vec<Rc<Object>>) -> JitFunction {
    JitFunction {
        ip: 0,
        constants,
        instructions,
        pointer: None,
        buffer: None,
        last_used_adr: String::new(),
    }
}

// TODO: Document this quite a bit more, as this is a little complicated
impl JitFunction {
    #[allow(dead_code)]
    pub fn compile(&mut self) -> bool {
        let mut ops = dynasmrt::x64::Assembler::new().unwrap();

        dynasm!(ops
            ; .arch x64
        );

        let offset = ops.offset();

        //let mut available_addresses = vec!["rcx".to_string(), "rbx".to_string(), "rax".to_string()];
/*
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

                    let number = &self.constants[idx as usize];

                    if let Object::Integer(number) = number.clone().deref() {
                        let adr = available_addresses.pop().unwrap();

                        self.last_used_adr = adr.to_string();

                        match adr.as_str() {
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

                    available_addresses.push(self.last_used_adr.clone());
                }
                OpCode::Multiply => {
                    dynasm!(ops
                        ; mul rbx
                    );

                    available_addresses.push(self.last_used_adr.clone());
                }
                OpCode::Return => {
                    dynasm!(ops
                        ; ret
                    );

                    available_addresses.push(self.last_used_adr.clone());
                }
                OpCode::Pop => {
                    available_addresses.push(self.last_used_adr.clone());
                }
                _ => {
                    return false;
                }
            };
        }
        */

        dynasm!(ops
            ; ret
        );

        let buf = ops.finalize().unwrap();

        let pointer = offset;

        self.pointer = Option::from(pointer);
        self.buffer = Option::from(buf);

        true
    }

    #[allow(dead_code)]
    pub fn run(&self) -> Object {
        let buf = self.buffer.as_ref().unwrap();

        let hello_fn: extern "win64" fn() -> i64 =
            unsafe { mem::transmute(buf.ptr(self.pointer.unwrap())) };

        Object::Integer(Integer { value: hello_fn() })
    }
}
