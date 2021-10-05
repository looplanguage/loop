use crate::compiler::Bytecode;
use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::read_uint16;
use crate::compiler::opcode::OpCode;
use crate::object::integer::Integer;
use crate::object::Object;

pub struct VM {
    stack: [Object; 2048],
    ip: i32,
    sp: u8,
    bytecode: Bytecode,
    last_popped: Option<Object>
}

pub fn build_vm(bt: Bytecode) -> VM {
    VM {
        stack: [Object::Integer(Integer{ value: 0}); 2048],
        ip: 0,
        sp: 0,
        bytecode: bt,
        last_popped: None
    }
}

impl VM {
    pub fn run(&mut self) -> Option<String> {
        while self.ip < (self.bytecode.instructions.len() as i32 ) - 1 {
            let _op = lookup_op(self.bytecode.instructions[self.ip as usize]);

            _op.as_ref()?;

            let op = _op.unwrap();

            self.ip += 1;
            match op {
                OpCode::Constant => {
                    let idx = read_uint16(self.bytecode.instructions[self.ip as usize..].to_owned());
                    self.ip += 2;

                    self.push(self.bytecode.constants[idx as usize].clone());
                }
                OpCode::Add => {}
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::Closure => {}
            }
        }

        None
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
        let popped = self.stack[self.sp as usize].clone();

        if self.sp <= 0 {
            panic!("can not pop nothing of the stack");
        } else {
            self.sp -= 1;
            self.last_popped = Some(popped);

            popped
        }
    }
}