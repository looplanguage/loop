mod frame;
pub(crate) mod function;
mod suffix;
mod tests;

use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{read_uint16, read_uint32, read_uint8};
use crate::compiler::opcode::OpCode;
use crate::compiler::Bytecode;
use crate::lib::config::CONFIG;
use crate::lib::exception::vm::VMException;
use crate::lib::jit::CodeGen;
use crate::lib::object::array::Array;
use crate::lib::object::builtin::BUILTINS;
use crate::lib::object::extension_method::EXTENSION_METHODS;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::hashmap::Hashmap;
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
use crate::lib::object::{Hashable, Object};
use crate::vm::frame::{build_frame, Frame};
use crate::vm::function::{run_function, run_function_stack};
use crate::vm::suffix::run_suffix_expression;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct VM {
    stack: Vec<Rc<RefCell<Object>>>,
    sp: u16,
    frames: Vec<Frame>,
    pub frame_index: usize,
    pub constants: Vec<Rc<RefCell<Object>>>,
    pub variables: HashMap<u32, Rc<RefCell<Object>>>,
    main_name: String,
}

const STACK_SIZE: usize = 2048;

pub struct VMState {
    variables: HashMap<u32, Rc<RefCell<Object>>>,
}

pub fn build_vm(bt: Bytecode, state: Option<&VMState>, main_name: String) -> VM {
    let mut stack = Vec::with_capacity(STACK_SIZE);

    for _ in 0..STACK_SIZE {
        stack.push(Rc::new(RefCell::new(Object::Null(Null {}))));
    }

    let default_frame = build_frame(
        Function {
            func: CompiledFunction {
                instructions: bt.instructions.clone(),
                num_locals: 0,
                num_parameters: 0,
                parsed_function: None,
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
            main_name,
        };
    }

    VM {
        stack,
        frames: vec![default_frame],
        frame_index: 0,
        sp: 0,
        constants: bt.constants,
        variables: HashMap::new(),
        main_name,
    }
}

impl VM {
    pub fn run(&mut self, codegen: Option<CodeGen>) -> Result<Rc<RefCell<Object>>, String> {
        if CONFIG.jit_enabled {
            let mut codegen = codegen.unwrap();
            let ptr = self.main_name.clone();
            let f = self.current_frame();
            let success = codegen.compile(f.func.func.clone(), ptr.clone(), self, Vec::new());

            if success {
                let returned = codegen.run(
                    ptr,
                    vec![Rc::from(RefCell::from(Object::Integer(Integer {
                        value: 0,
                    })))],
                );

                return Ok(Rc::from(RefCell::from(returned)));
            } else {
                return Err(String::from("Unable to JIT main"));
            }
        }

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

                    self.push(variable);

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
                    let popped = self.pop();
                    let condition = popped.borrow();

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
                    let vm_exception = run_function(self, args, CONFIG.jit_enabled);

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
                            VMException::EmptyArray => {
                                Err(String::from("array index does not exist."))
                            }
                            VMException::UnableToJIT => Err(String::from(
                                "Unable to JIT function! (You're running JIT-mode)",
                            )),
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

                    let builtin_function = Rc::new(RefCell::new(BUILTINS[ct].builtin.clone()));

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

                    let popped = self.pop();

                    let mut params: Vec<Object> = vec![];

                    for _n in 0.._parameters {
                        let item = self.pop();

                        let item_dereffed = &*item.borrow();

                        let obj = item_dereffed.clone();

                        params.push(obj);
                    }

                    params.reverse();

                    let method = &EXTENSION_METHODS[method_id];

                    let push = (method.builtin)(popped, params);

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
                            VMException::EmptyArray => {
                                Err(String::from("array index does not exist."))
                            }
                            VMException::UnableToJIT => Err(String::from(
                                "Unable to JIT function! (You're running JIT-mode)",
                            )),
                        };
                    }

                    let object = push.ok().unwrap();

                    self.push(Rc::from(RefCell::from(object)));

                    None
                }
                OpCode::Array => {
                    let ip = self.current_frame().ip;

                    let element_amount =
                        read_uint16(&self.current_frame().instructions()[ip as usize..]);

                    self.increment_ip(2);

                    let mut elements: Vec<Rc<RefCell<Object>>> = Vec::new();

                    for _i in 0..element_amount {
                        let element = self.pop();
                        elements.insert(0, element.clone());
                    }

                    let array = Object::Array(Array { values: elements });

                    self.push(Rc::from(RefCell::from(array)))
                }
                OpCode::Index => {
                    let index = self.pop();
                    let indexed = self.pop();

                    if let Object::Array(array) = &*indexed.borrow() {
                        if let Object::Integer(id) = &*index.borrow() {
                            let item = array.values.get(id.value as usize);

                            if let Some(item) = item {
                                self.push(item.clone());
                            } else {
                                self.push(Rc::from(RefCell::from(Object::Null(Null {}))));
                            }
                        }
                    }

                    if let Object::Hashmap(hashmap) = &*indexed.borrow() {
                        let hashable = index.borrow().get_hash();

                        if let Some(hash) = hashable {
                            let item = hashmap.values.get(&hash);

                            if let Some(item) = item {
                                self.push(item.clone());
                            } else {
                                self.push(Rc::from(RefCell::from(Object::Null(Null {}))));
                            }
                        }
                    }

                    None
                }
                OpCode::AssignIndex => {
                    let value = self.pop();
                    let index = self.pop();
                    let array = self.pop();

                    if let Object::Array(arr) = &*array.as_ref().borrow() {
                        if let Object::Integer(index) = &*index.as_ref().borrow() {
                            *arr.values[index.value as usize].borrow_mut() =
                                value.as_ref().borrow().clone();
                        }
                    }

                    if let Object::Hashmap(hashmap) = &mut *array.as_ref().borrow_mut() {
                        let hashable = index.borrow().get_hash();
                        if let Some(index) = hashable {
                            hashmap.values.insert(index, value.clone());
                        }
                    }

                    self.push(value.clone());

                    None
                }
                OpCode::Hashmap => {
                    let ip = self.current_frame().ip;

                    let element_amount =
                        read_uint16(&self.current_frame().instructions()[ip as usize..]);

                    self.increment_ip(2);

                    let mut elements: HashMap<Hashable, Rc<RefCell<Object>>> = HashMap::new();

                    for _i in 0..element_amount {
                        let value = self.pop();
                        let key = self.pop();

                        let hashable = key.borrow().get_hash();

                        if let Some(hash) = hashable {
                            elements.insert(hash, value.clone());
                        }
                    }

                    self.push(Rc::from(RefCell::from(Object::Hashmap(Hashmap {
                        values: elements,
                    }))));

                    None
                }
                OpCode::Pow => run_suffix_expression(self, "^"),
                OpCode::And => run_suffix_expression(self, "&&"),
                OpCode::Or => run_suffix_expression(self, "||"),
                // Sections are completely ignored by the VM, they serve as helpers for the JIT engine
                OpCode::StartSection => {
                    self.increment_ip(2);

                    None
                }
                OpCode::EndSection => None,
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

    pub fn push(&mut self, obj: Rc<RefCell<Object>>) -> Option<String> {
        if (self.sp + 1) as usize >= 2048 {
            panic!("stack overflow")
        }

        self.stack[self.sp as usize] = obj;

        self.sp += 1;

        None
    }

    pub fn pop(&mut self) -> Rc<RefCell<Object>> {
        let popped = self.stack[self.sp as usize - 1].to_owned();
        self.sp -= 1;

        popped
    }
}
