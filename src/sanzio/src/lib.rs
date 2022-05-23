#[cfg(feature = "mlua")]
use mlua::{Lua, MultiValue, Value};
use std::ffi::CStr;
use std::ops::Deref;
use std::os::raw::c_char;
use std::str;
use vinci::ast::instructions::memory::LoadType;
use vinci::ast::instructions::suffix::BinaryOperation;
use vinci::ast::instructions::Node;
use vinci::types::ValueType;

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
    libs: Vec<String>,
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
            code: String::from("ffi = require(\"ffi\")\n"),
            doing_statement: false,
            libs: vec![],
        }
    }

    fn add_code(&mut self, code: String) {
        self.code.push_str(code.as_str());
    }

    fn add_code_str(&mut self, code: &str) {
        self.code.push_str(code);
    }

    fn add_constant_value(&mut self, value: &ValueType) {
        match value {
            ValueType::Void => self.add_code_str("null"),
            ValueType::Integer(i) => self.add_code(i.to_string()),
            ValueType::Boolean(b) => self.add_code(b.to_string()),
            ValueType::Character(c) => self.add_code(format!("\"{}\"", c)),
            ValueType::Float(f) => self.add_code(f.to_string()),
            ValueType::Array(a) => {
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

                        self.add_constant_value(item);

                        if index != items.len() {
                            self.add_code_str(",")
                        }
                    }

                    self.add_code_str("}");
                }
            }
        }
    }

    fn add_library(&mut self, lib_name: String) {
        if self.libs.contains(&lib_name) {
            println!("Library is already loaded");
        }
        self.libs.push(lib_name);
    }

    fn compile_node(&mut self, node: &Node) {
        match node {
            Node::CONSTANT(cst) => {
                self.add_constant_value(cst);
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
                self.compile_node(store.value.deref());
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
            Node::WHILE(whi) => {
                self.add_code_str("(function() while ");
                self.compile_node(&whi.condition);
                self.add_code_str(" do ");

                self.compile_nodes(&whi.body);
                self.add_code_str(" end end)()");
            }
            Node::INDEX(idx) => {
                self.compile_node(&idx.to_index);
                self.add_code_str("[");
                self.compile_node(&idx.index);
                self.add_code_str("+ 1]");
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
            Node::LOADLIB(lib) => {
                if let Ok(str) = self.get_lib_signiture(lib.clone().get_path().to_string()) {
                    self.add_library(lib.clone().get_path());
                    self.add_code(format!("ffi.cdef[[ {} ]]", str.as_str()));
                    self.add_code(format!(
                        "{} = ffi.load(\"{}\")",
                        lib.namespace,
                        lib.clone().get_path()
                    ))
                } else {
                    panic!("Somethings went wrong during loading of library");
                }
            }
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
            Node::LIBCALL(a) => {
                let x: Vec<&str> = a.namespace.split("::").collect();

                self.add_code(format!("{}.{}(", x[0], x[1]));
                let mut index = 0;
                for argument in &a.arguments {
                    index += 1;
                    self.compile_node(argument);

                    if index != a.arguments.len() {
                        self.add_code_str(",");
                    }
                }
                self.add_code_str(")");
            }
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
                self.add_code_str(";")
            }
        }
    }

    fn get_lib_signiture(&self, path: String) -> Result<String, ()> {
        let full_path: String = if std::env::consts::OS.to_string() == "windows" {
            format!("{}.dll", path)
        } else {
            format!("{}.so", path)
        };
        let lib = libloading::Library::new(full_path);
        if let Ok(l) = lib {
            unsafe {
                if let Ok(sym) = l.get(b"library_signatures") {
                    let func: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> = sym;
                    let str = CStr::from_ptr(func()).to_str().unwrap().to_owned();
                    return Ok(str);
                }
                return Err(());
            }
        }
        Err(())
    }
}
