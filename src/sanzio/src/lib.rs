#[cfg(feature = "mlua")]
use mlua::{Lua, MultiValue, Value};

use std::ops::Deref;

use std::collections::HashMap;
use std::str;
use vinci::ast::instructions::memory::LoadType;
use vinci::ast::instructions::suffix::BinaryOperation;
use vinci::ast::instructions::Node;
use vinci::types::{Type, ValueType};
mod exception;
use exception::throw_runtime_exception;

pub struct Sanzio {
    #[cfg(feature = "mlua")]
    lua: Lua,
}

#[cfg(feature = "mlua")]
pub fn parse_multivalue(m: MultiValue) -> vinci::types::ValueType {
    let vec = m.into_vec();
    let value = vec.first().unwrap_or(&Value::Nil);

    match value {
        Value::Boolean(b) => ValueType::Boolean(*b),
        Value::Integer(n) => ValueType::Integer(*n as i64),
        Value::Number(n) => ValueType::Integer(n.round() as i64),
        Value::String(str) => {
            let chars: Vec<ValueType> = str
                .to_str()
                .unwrap()
                .chars()
                .map(ValueType::Character)
                .collect();

            ValueType::Array(Box::new(chars))
        }
        _ => ValueType::Void,
    }
}

impl Default for Sanzio {
    fn default() -> Self {
        unsafe { Sanzio::new() }
    }
}

impl Sanzio {
    /// # Safety
    /// This function is unsafe due to Sanzio allowing C FFI. And FFI is inherintly unsafe due to
    /// the fact that C code is unsafe.
    pub unsafe fn new() -> Sanzio {
        Sanzio {
            #[cfg(feature = "mlua")]
            lua: Lua::unsafe_new(),
        }
    }

    #[cfg(feature = "mlua")]
    pub fn run(&mut self, ast: vinci::ast::AST) -> MultiValue {
        use std::process::exit;

        let mut backend = LuaBackend::new();

        backend.compile_nodes_global(&ast.nodes);

        let result = self.lua.load(backend.code.as_str()).eval::<MultiValue>();

        if let Err(result) = result {
            println!("{}", result);
            exit(1);
        };

        result.unwrap()
    }

    pub fn compile_to_lua(ast: &vinci::ast::AST) -> String {
        let mut backend = LuaBackend::new();
        backend.compile_nodes_global(&ast.nodes);

        backend.code
    }
}

struct LuaBackend {
    code: String,
    doing_statement: bool,
    library_paths: Vec<String>,
    library_names: Vec<String>,
    /// All the variables key is index, value of type of variable
    variable_table: HashMap<u64, Type>,
}

impl LuaBackend {
    /// code: String::from(
    ///      "ffi = require(\"ffi\")
    ///      ffi.cdef[[int dub(int i);]]
    ///      clib = ffi.load(\"C:/Users/woute/Documents/LoopLanguage/code/loop/lib\")
    ///      print(clib.dub(2))",
    ///  ),
    pub fn new() -> LuaBackend {
        LuaBackend {
            #[cfg(feature = "libloading")]
            code: String::from("ffi = require(\"ffi\")\n"),
            #[cfg(not(feature = "libloading"))]
            code: String::from(""),
            doing_statement: false,
            library_paths: vec![],
            library_names: vec![],
            variable_table: HashMap::new(),
        }
    }

    fn add_code(&mut self, code: String) {
        self.code.push_str(code.as_str());
    }

    fn add_code_str(&mut self, code: &str) {
        self.code.push_str(code);
    }

    fn add_constant_value(&mut self, value: &ValueType, index: Option<u64>) {
        match value {
            ValueType::Void => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code_str("null")
            }
            ValueType::Integer(i) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code(i.to_string())
            }
            ValueType::Boolean(b) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code(b.to_string())
            }
            ValueType::Character(c) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code(format!("\"{}\"", c))
            }
            ValueType::Float(f) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code(f.to_string())
            }
            ValueType::Function(_, args, id, block) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code_str("(function");

                self.add_code_str("(");

                let mut index = 0;
                for _ in args.clone().iter() {
                    self.add_code(format!("param_{}_{}", id, index));
                    index += 1;

                    if index != args.len() {
                        self.add_code_str(",")
                    }
                }

                self.add_code_str(") ");

                self.compile_nodes(block);

                self.add_code_str("end)");
            }
            ValueType::Compound(_, values) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                self.add_code_str("({");

                for (key, value) in values.iter().enumerate() {
                    self.add_code(format!("[{} + 1] = ", key));
                    self.add_constant_value(value, None);

                    if key + 1 != values.len() {
                        self.add_code_str(",");
                    }
                }

                self.add_code_str("})");
            }
            ValueType::Array(a) => {
                if let Some(idx) = index {
                    self.variable_table.insert(idx, value.clone().to_type());
                }

                let items = a.deref();

                if let Some(ValueType::Character(_)) = a.get(0) {
                    let mut string = String::from("");

                    for item in items {
                        if let ValueType::Character(char) = item {
                            if *char == '\n' {
                                string.push('\\')
                            }

                            string.push(*char);
                        }
                    }

                    self.add_code(format!("\"{}\"", string))
                } else {
                    self.add_code_str("{");

                    let mut index = 0;
                    for item in items {
                        index += 1;

                        self.add_constant_value(item, None);

                        if index != items.len() {
                            self.add_code_str(",")
                        }
                    }

                    self.add_code_str("}");
                }
            }
        }
    }

    fn compile_node(&mut self, node: &Node) {
        match node {
            Node::CONSTANT(cst) => {
                self.add_constant_value(cst, None);
            }
            Node::LOAD(l) => {
                match l.load_type {
                    LoadType::VARIABLE => self.add_code(format!("var_{}", l.index)),
                    LoadType::PARAMETER(unique_identifier) => {
                        self.add_code(format!("param_{}_{}", unique_identifier, l.index))
                    }
                };
            }
            Node::STORE(store) => {
                self.doing_statement = true;
                self.add_code(format!("var_{} = ", store.index));
                if let Node::CONSTANT(v) = *store.value.clone() {
                    self.add_constant_value(&v, Some(store.index));
                } else {
                    self.compile_node(store.value.deref());
                }

                self.doing_statement = false;
            }
            Node::SUFFIX(suffix) => {
                self.add_code_str("(");
                self.compile_node(&suffix.left);

                match suffix.operation {
                    BinaryOperation::ADD => self.add_code_str("+"),
                    BinaryOperation::SUBTRACT => self.add_code_str("-"),
                    BinaryOperation::MULTIPLY => self.add_code_str("*"),
                    BinaryOperation::DIVIDE => self.add_code_str("/"),
                    BinaryOperation::POWER => self.add_code_str("^"),
                    BinaryOperation::GREATERTHAN => self.add_code_str(">"),
                    BinaryOperation::EQUALS => self.add_code_str("=="),
                    BinaryOperation::NOTEQUALS => self.add_code_str("!="),
                    BinaryOperation::MODULO => self.add_code_str("%"),
                };

                self.compile_node(&suffix.right);
                self.add_code_str(")");
            }
            Node::CONDITIONAL(cond) => {
                let stmt = self.doing_statement;

                if stmt {
                    self.add_code_str("(function() if ");
                } else {
                    self.add_code_str("if ");
                }

                self.compile_node(&cond.condition);

                self.add_code_str(" then ");

                self.compile_nodes(&cond.body);

                if !cond.alternative.is_empty() {
                    self.add_code_str(" else ");

                    self.compile_nodes(&cond.alternative);
                }

                if stmt {
                    self.add_code_str(" end end)()")
                } else {
                    self.add_code_str(" end")
                }
            }
            Node::FUNCTION(func) => {
                self.variable_table
                    .insert(func.unique_identifier as u64, func.return_type.clone());

                if func.name.is_empty() {
                    self.add_code_str("(");
                }

                self.add_code_str("function");

                if !func.name.is_empty() {
                    self.add_code(format!(" {}", func.name))
                }

                self.add_code_str("(");

                let mut index = 0;
                for _ in &func.parameters {
                    self.add_code(format!("param_{}_{}", func.unique_identifier, index));
                    index += 1;

                    if index != func.parameters.len() {
                        self.add_code_str(",")
                    }
                }

                self.add_code_str(") ");

                self.compile_nodes(&func.body);

                self.add_code_str("end");

                if func.name.is_empty() {
                    self.add_code_str(")")
                }
            }
            Node::CALL(call) => {
                let namespace = &call.call;
                // Calling a function from an import (DLL or Loop)
                if let Node::CONSTANT(e) = namespace {
                    // You get an Lua output like this
                    //
                    // res = (function()
                    //     local res = std.input(var_0)
                    //     if type(res) == "cdata" then
                    //         return ffi.string(res)
                    //     else
                    //       return res
                    //     end
                    // end)()

                    let str = e.clone().char_arr_to_string();
                    let parts: Vec<&str> = str.split("::").collect();
                    if parts[1] == "println" || parts[1] == "print" {
                        self.add_code(format!("{}.{}(", parts[0], parts[1]))
                    } else {
                        self.add_code(format!(
                            "(function() local res = {}.{}(",
                            parts[0], parts[1]
                        ));
                    }
                    let mut index = 0;
                    for argument in &call.arguments {
                        index += 1;
                        self.compile_node(argument);

                        if index != call.arguments.len() {
                            self.add_code_str(",");
                        }
                    }
                    if parts[1] == "println" || parts[1] == "print" {
                        self.add_code_str(")");
                    } else {
                        self.add_code(") if type(res) == \"cdata\" then return ffi.string(res) else return res end end)()".to_string());
                    }
                    // Calling a user-defined function or a class
                } else {
                    self.compile_node(&call.call);

                    self.add_code_str("(");

                    let mut index = 0;
                    for argument in &call.arguments {
                        index += 1;
                        self.compile_node(argument);

                        if index != call.arguments.len() {
                            self.add_code_str(",");
                        }
                    }

                    self.add_code_str(")");
                }
            }
            Node::WHILE(whi) => {
                self.add_code_str("(function() while ");
                self.compile_node(&whi.condition);
                self.add_code_str(" do ");

                self.compile_nodes(&whi.body);
                self.add_code_str(" end end)()");
            }
            Node::INDEX(idx) => {
                // Works on string and on arrays
                if let Node::CONSTANT(valuetype) = *idx.to_index.clone() {
                    if let ValueType::Array(arr) = valuetype {
                        if let ValueType::Character(_) = (*arr)[0].clone() {
                            // Indexing a raw value:
                            // "hello"[1]
                            self.add_code_str("string.sub(");
                            self.compile_node(&idx.to_index);
                            self.add_code_str(", ");
                            self.compile_node(&idx.index);
                            self.add_code_str(" + 1, ");
                            self.compile_node(&idx.index);
                            self.add_code_str(" + 1)");
                            return;
                        }
                    }
                } else if let Node::LOAD(l) = *idx.to_index.clone() {
                    // This if-statement is to check if variable does not exist.
                    // If that is the case it is a field of a class, and needs to be indexed the regular way
                    if self.variable_table.get(&l.index).is_some() {
                        //println!("{:?}", self.variable_table.get(&l.index));
                        if let Type::ARRAY(_type) =
                            self.variable_table.get(&l.index).unwrap().clone()
                        {
                            if let Type::CHAR = *_type {
                                // Indexing a variable
                                // i := "hello"
                                // i[1]
                                self.add_code_str("string.sub(");
                                self.compile_node(&idx.to_index);
                                self.add_code_str(", ");
                                self.compile_node(&idx.index);
                                self.add_code_str(" + 1, ");
                                self.compile_node(&idx.index);
                                self.add_code_str(" + 1)");
                                return;
                            }
                        }
                    }
                }
                // Indexing a regular array
                self.compile_node(&idx.to_index);
                self.add_code_str("[");
                self.compile_node(&idx.index);
                self.add_code_str(" + 1]");
            }
            Node::SLICE(slice) => {
                self.add_code_str("({unpack(");
                self.compile_node(&*slice.to_slice);
                self.add_code_str(",");
                self.compile_node(&*slice.from);
                self.add_code_str("+1,");
                self.compile_node(&*slice.to);
                self.add_code_str("+1)})");
            }
            Node::PUSH(push) => {
                self.add_code_str("table.insert(");
                self.compile_node(&*push.to_push);
                self.add_code_str(",");
                self.compile_node(&*push.item);
                self.add_code_str(")")
            }
            Node::COPY(_) => {}
            Node::LOADLIB(lib) => match self.get_lib_signiture(lib.clone().get_path()) {
                Ok(str) => {
                    let extension = if cfg!(windows) { "dll" } else { "so" };

                    self.add_library_path(lib.clone().get_path());
                    self.add_library_namespace(lib.clone().namespace);
                    self.add_code(format!("ffi.cdef[[ {} ]]\n", str.as_str()));
                    self.add_code(format!(
                        "{} = ffi.load(\"{}.{}\")",
                        lib.namespace,
                        lib.clone().get_path(),
                        extension
                    ))
                }
                Err(str) => {
                    throw_runtime_exception(str, None);
                    unreachable!("Loadlib should crash at this point")
                }
            },
            Node::RETURN(rt) => {
                self.doing_statement = true;
                self.add_code_str("return ");
                self.compile_node(rt);
                self.doing_statement = false;
            }
            Node::ASSIGN(to, item) => {
                self.compile_node(to);
                self.add_code_str(" = ");
                self.compile_node(item);
            }
            Node::POP(from, id) => {
                self.add_code_str("table.remove(");
                self.compile_node(from);
                self.add_code_str(",");
                self.compile_node(id);
                self.add_code_str("+ 1)")
            }
            Node::LENGTH(item) => {
                self.add_code_str("#");
                self.compile_node(item)
            }
            Node::AND(a, b) => {
                self.add_code_str("(");
                self.compile_node(a);
                self.add_code_str(") and (");
                self.compile_node(b);
                self.add_code_str(")");
            }
            Node::OR(a, b) => {
                self.add_code_str("(");
                self.compile_node(a);
                self.add_code_str(") or (");
                self.compile_node(b);
                self.add_code_str(")");
            }
            Node::COMPOUND(_) => (),
        }
    }

    fn compile_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.compile_node(node);
            self.add_code_str(";")
        }
    }

    fn compile_nodes_global(&mut self, nodes: &[Node]) {
        let mut index = 0;
        for node in nodes {
            index += 1;

            if index == nodes.len() {
                match node.clone() {
                    Node::STORE(_) => {}
                    Node::ASSIGN(_, _) => {}
                    _ => self.add_code_str("print("),
                };
            }

            self.compile_node(node);

            if index == nodes.len() {
                match node.clone() {
                    Node::STORE(_) => {}
                    Node::ASSIGN(_, _) => {}
                    _ => self.add_code_str(")"),
                };
            }

            if index != nodes.len() {
                let add_colon = !matches!(node, Node::COMPOUND(_));

                if add_colon {
                    self.add_code_str(";")
                }
            }
        }
    }

    fn add_library_path(&mut self, lib_path: String) {
        if self.library_paths.contains(&lib_path) {
            println!("RuntimeWarning: Library is already loaded");
        }
        self.library_paths.push(lib_path);
    }

    fn add_library_namespace(&mut self, lib_name: String) {
        if self.library_names.contains(&lib_name) {
            throw_runtime_exception(
                format!("The libary alias {}, is already in use", lib_name),
                None,
            );
        }

        self.library_names.push(lib_name);
    }

    fn get_lib_signiture(&self, _path: String) -> Result<String, String> {
        #[cfg(feature = "libloading")]
        {
            use std::ffi::CStr;
            use std::os::raw::c_char;

            let full_path = if std::env::consts::OS == "windows" {
                format!("{}.dll", _path)
            } else {
                format!("{}.so", _path)
            };

            let lib = libloading::Library::new(full_path.clone());
            if let Ok(l) = lib {
                unsafe {
                    let signature = l.get(b"library_signatures");
                    if let Ok(sym) = signature {
                        let func: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> = sym;
                        let str = CStr::from_ptr(func()).to_str().unwrap().to_owned();
                        return Ok(str);
                    } else if let Err(_) = signature {
                        return Err(String::from(
                            "Could not call the 'library_signatures' function",
                        ));
                    } else {
                        unreachable!("get_lib_signatures is not supposed to get here")
                    }
                }
            } else {
                return Err(format!("The library: {}, does not exist", full_path));
            }
        }
        #[allow(unreachable_code)]
        Err(String::from("Loading of dynamic libaries is not enabled"))
    }
}
