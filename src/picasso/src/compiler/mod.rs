//! Responsible for transpiling Loop to D
pub mod compile;
mod modifiers;
mod symbol_table;
mod test;

use crate::compiler::compile::expression_array::compile_expression_array;
use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_call::compile_expression_call;
use crate::compiler::compile::expression_conditional::compile_expression_conditional;
use crate::compiler::compile::expression_float::compile_expression_float;
use crate::compiler::compile::expression_function::{compile_expression_function, Function};
use crate::compiler::compile::expression_hashmap::compile_expression_hashmap;
use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::compile::expression_index::{
    compile_expression_assign_index, compile_expression_index, compile_expression_slice,
};
use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_loop::{
    compile_loop_array_iterator_expression, compile_loop_expression,
    compile_loop_iterator_expression,
};
use crate::compiler::compile::expression_null::compile_expression_null;
use crate::compiler::compile::expression_string::compile_expression_string;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::compile::statement_break::compile_break_statement;
use crate::compiler::compile::statement_class::compile_class_statement;
use crate::compiler::compile::statement_constant_declaration::compile_statement_constant_declaration;
use crate::compiler::compile::statement_extend::compile_extend_statement;
use crate::compiler::compile::statement_import::compile_import_statement;
use crate::compiler::compile::statement_return::compile_return_statement;
use crate::compiler::compile::statement_variable_assign::compile_statement_variable_assign;
use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::modifiers::Modifiers;
use crate::compiler::symbol_table::{
    build_deeper_variable_scope, build_variable_scope, Symbol, SymbolScope,
};
use crate::exception::compiler::CompilerException;
use crate::exception::compiler_new::CompilerError;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::block::Block;
use crate::parser::statement::class::Method;
use crate::parser::statement::Statement;
use crate::parser::types::{Compound, Types};
use crate::{lexer, parser};
use colored::Colorize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The result of the transpiler, which will be passed to the D compiler [crate::util::execute_code]
pub struct Arc {
    pub imports: Vec<String>,
    pub functions: HashMap<String, Function>,
}

impl Arc {
    pub fn get_arc(&self) -> String {
        let mut code = String::new();

        for function in &self.functions {
            if function.0.as_str() == "main" {
                code.push_str(function.1.code.as_str());
            }
        }

        code
    }
}

/// The compiler itself containing global metadata needed during compilation and methods
pub struct Compiler {
    pub scope_index: i32,
    // Module, Scope
    pub symbol_scope: HashMap<String, Rc<RefCell<SymbolScope>>>,
    pub variable_count: u32,
    pub last_extension_type: Option<Expression>,
    pub location: String,
    pub locations: Vec<String>,
    pub export_name: String,
    pub prev_location: String,
    pub breaks: Vec<u32>,

    pub imports: Vec<String>,
    pub functions: HashMap<String, Function>,
    pub function_stack: Vec<String>,
    pub function_count: i32,
    pub current_function: String,
    // Extensions to basetypes
    pub extensions: HashMap<String, Vec<Method>>,
    // Specifies whether or not compiling should add code
    pub dry: u32,
    pub base_location: String,
    pub compiled_from: String,
}

#[derive(Clone)]
pub struct CompilerState {
    pub variable_scope: HashMap<String, Rc<RefCell<SymbolScope>>>,
    pub variable_count: u32,
    pub function_count: i32,
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            scope_index: 0,
            variable_count: 0,
            symbol_scope: HashMap::from([(
                "".to_string(),
                Rc::new(RefCell::new(build_variable_scope())),
            )]),
            last_extension_type: None,
            location: String::new(),
            export_name: String::new(),
            prev_location: String::new(),
            breaks: Vec::new(),
            extensions: HashMap::new(),
            locations: vec![],
            imports: Vec::new(),
            functions: HashMap::from([(
                "main".to_string(),
                Function {
                    name: "main".to_string(),
                    code: String::from(""),
                    parameters: Vec::new(),
                    return_type: Types::Void,
                },
            )]),
            function_stack: Vec::new(),
            function_count: 0,
            current_function: String::from("main"),
            dry: 0,
            base_location: "".to_string(),
            compiled_from: "".to_string(),
        }
    }
}

impl Compiler {
    /// Main compilation function which compiles a syntax tree from the parser into Arc
    ///
    /// # Example
    /// ```loop
    /// let lexer = lexer::build_lexer("var x = 100");
    /// let mut parser = parser::build_parser(lexer);
    /// let program = parser.parse();
    ///
    /// let compiler = Compiler::default();
    ///
    /// let result = compiler.compile(program);
    ///
    /// if result.is_err() {
    ///     // Handle error
    /// }
    /// ```
    pub fn compile(&mut self, program: Program) -> Result<Arc, CompilerException> {
        for statement in program.statements {
            let err = self.compile_statement(statement);

            #[allow(clippy::single_match)]
            match err {
                Err(exception) => {
                    self.print_error(exception.clone());
                    return Err(exception);
                }
                _ => (),
            }
        }

        Ok(self.get_arc())
    }

    fn print_error(&self, error: CompilerException) {
        let mut width = String::new();

        for _ in 0..error.location.1.to_string().len() {
            width.push(' ');
        }

        // Should be replaced by a match if different requirements
        let minus = 2;

        let colon = error.location.1 - minus;

        println!("{}", "CompilerException".red());
        println!(
            "{} | -> {} [{}:{}]",
            width, self.location, error.location.0, colon
        );

        println!("{} | ", width);
        println!(
            "{} | {}",
            error.location.0,
            self.compiled_from
                .lines()
                .nth((error.location.0 - 1) as usize)
                .unwrap()
        );

        let spaces = colon;

        let mut cursor_width = String::new();

        for _ in 0..(spaces - (width.len() as i32)) {
            cursor_width.push(' ');
        }

        println!("{} | {}{}", width, cursor_width, "^".red());

        println!("{} | ", width);

        println!("{} = {}", width, format!("{}", error).blue());
    }

    /// Enters a compilation "location" aka a module. A module has its own variable scope and thus
    /// when importing a file this file is completely seperate from the previous location. A stack
    /// is used to keep track of all locations(read modules).
    pub fn enter_location(&mut self, location: String) {
        self.enter_symbol_scope();

        self.locations.push(location.clone());
        self.symbol_scope.insert(
            location.clone(),
            Rc::new(RefCell::new(build_variable_scope())),
        );
        self.location = location;
    }

    /// Exits a compilation "location" aka a module. When exiting a location it pops the last
    /// location from the stack.
    pub fn exit_location(&mut self) -> String {
        let last_loc = self.location.clone();

        // This is needed as if we are only one location "deep" the previous location wont exist,
        // so we set the location in the else block to "" which is the default root location.
        if self.locations.len() > 1 {
            self.location = self.locations.pop().unwrap();
        } else {
            self.location = "".to_string();
        }

        self.exit_symbol_scope();

        last_loc
    }

    /// Allows you to use Loop code within the compiler
    pub fn compile_generic_loop(&mut self, str: &str) -> Result<Arc, CompilerException> {
        let lexer = lexer::build_lexer(str);
        let mut parser = parser::build_parser(lexer, self.location.as_str());

        let program = parser.parse()?;

        self.compile(program)
    }

    /// Will go one scope deeper in dry compiling. Dry compiling means that you can get the result
    /// of a compilation without it actually generating any effects.
    pub fn drier(&mut self) {
        self.dry += 1;
    }

    /// Will go one scope shallower in dry compilation. Dry compiling means that you can get the result
    /// of a compilation without it actually generating any effects.
    pub fn undrier(&mut self) {
        self.dry -= 1;
    }

    pub fn get_compound_type(&self, name: &str) -> Option<Types> {
        let class = self.resolve_symbol(&name.to_string());

        if let Some(class) = class {
            if let Types::Compound(Compound(name, values)) = class._type {
                // Instantiate the class using a constant
                return Some(Types::Compound(Compound(name, values)));
            }

            if let Types::Auto = class._type {
                return Some(Types::Auto);
            }
        }

        None
    }

    pub fn default_with_state(compiler_state: CompilerState) -> Compiler {
        Compiler {
            function_count: compiler_state.function_count,
            variable_count: compiler_state.variable_count,
            symbol_scope: compiler_state.variable_scope,
            ..Compiler::default()
        }
    }

    pub fn get_compiler_state(&self) -> CompilerState {
        CompilerState {
            function_count: self.function_count,
            variable_count: self.variable_count,
            variable_scope: self.symbol_scope.clone(),
        }
    }

    /// Adds code to the current function compilation scope
    pub fn add_to_current_function(&mut self, code: String) {
        if self.dry == 0 {
            let func = self.functions.get_mut(&*self.current_function);

            func.unwrap().code.push_str(code.as_str());
        }
    }

    /// Allows replacing context
    pub fn replace_at_current_function(&mut self, replace: String, with: String) {
        if self.dry == 0 {
            let func = self.functions.get_mut(&*self.current_function);

            let unwrapped = func.unwrap();
            let replaced = unwrapped.code.replace(replace.as_str(), &*with);

            unwrapped.code = replaced;
        }
    }

    fn get_symbol_scope(&self) -> Rc<RefCell<SymbolScope>> {
        return (*self.symbol_scope.get(&self.location).as_ref().unwrap()).clone();
    }

    fn get_symbol_mutable(
        &self,
        index: u32,
        name: String,
        loc: Option<String>,
    ) -> Option<Rc<RefCell<Symbol>>> {
        self.symbol_scope
            .get(&*loc.unwrap_or_else(|| self.location.clone()))
            .unwrap()
            .as_ref()
            .borrow_mut()
            .get_variable_mutable(index, name)
    }

    /// Enter a deeper variable scope
    pub fn enter_symbol_scope(&mut self) {
        let scope = build_deeper_variable_scope(Option::from(self.get_symbol_scope()));
        self.scope_index += 1;
        *self.symbol_scope.get_mut(&self.location).unwrap() = Rc::from(RefCell::from(scope));
    }

    /// Exit a variable scope and go one shallower
    pub fn exit_symbol_scope(&mut self) {
        let outer = self
            .symbol_scope
            .get(&self.location)
            .as_ref()
            .unwrap()
            .as_ref()
            .borrow()
            .outer
            .clone();
        self.scope_index -= 1;

        if let Some(outer) = outer {
            *self.symbol_scope.get_mut(&self.location).unwrap() = outer;
        }
    }

    pub fn get_arc(&self) -> Arc {
        Arc {
            functions: self.functions.clone(),
            imports: self.imports.clone(),
        }
    }

    /// Compiles the [Expression] [Node](crate::parser::program::Node)
    ///
    /// # Examples
    /// ```loop
    /// let mut compiler = Compiler::default();
    /// let exp = Expression::Integer(Integer { value: 10 });
    ///
    /// let result = compiler.compile_expression(exp);
    /// ```
    fn compile_expression(&mut self, expr: Expression) -> Result<Types, CompilerException> {
        match expr {
            Expression::Identifier(identifier) => compile_expression_identifier(self, identifier),
            Expression::Integer(int) => compile_expression_integer(self, int),
            Expression::Suffix(suffix) => compile_expression_suffix(self, *suffix),
            Expression::Boolean(boolean) => compile_expression_boolean(self, boolean),
            Expression::Function(func) => compile_expression_function(self, func),
            Expression::Conditional(conditional) => {
                compile_expression_conditional(self, *conditional)
            }
            Expression::Null(_) => compile_expression_null(self),
            Expression::Call(call) => compile_expression_call(self, call),
            Expression::Float(float) => compile_expression_float(self, float),
            Expression::String(string) => compile_expression_string(self, string),
            Expression::Index(index) => compile_expression_index(self, *index),
            Expression::Array(array) => compile_expression_array(self, *array),
            Expression::AssignIndex(assign) => compile_expression_assign_index(self, *assign),
            Expression::Loop(lp) => compile_loop_expression(self, lp),
            Expression::LoopIterator(lp) => compile_loop_iterator_expression(self, lp),
            Expression::LoopArrayIterator(lp) => compile_loop_array_iterator_expression(self, lp),
            Expression::Hashmap(hash) => compile_expression_hashmap(self, hash),
            Expression::Slice(slice) => compile_expression_slice(self, slice),
        }
    }

    /// Compiles a loop [Block], this differs from [Compiler::compile_block] in that it wont add curly braces
    fn compile_loop_block(&mut self, block: Block) -> Result<Types, CompilerException> {
        let mut return_type = Types::Void;
        self.enter_symbol_scope();

        let mut index = 0;
        for statement in block.statements.clone() {
            index += 1;

            let err = self.compile_statement(statement.clone());

            // If its either a return statement, or the last statement is an expression than that is the return type of this block
            if let Statement::Return(_) = statement {
                if let Ok(_type) = err.clone() {
                    return_type = _type;
                }
            }

            if index == block.statements.len() {
                if let Statement::Expression(_) = statement {
                    if let Ok(_type) = err.clone() {
                        return_type = _type;
                    }
                }
            }

            #[allow(clippy::single_match)]
            match &err {
                Err(_exception) => return Err(_exception.clone()),
                _ => (),
            }
        }

        self.exit_symbol_scope();

        Ok(return_type)
    }

    /// Defines a new variable and increases the amount of variables that exist
    fn define_symbol(&mut self, name: String, var_type: Types, parameter_id: i32) -> Symbol {
        let var = self
            .symbol_scope
            .get_mut(&self.location)
            .unwrap()
            .as_ref()
            .borrow_mut()
            .define(
                self.variable_count,
                name,
                var_type,
                Modifiers::new(false, self.location.clone(), false),
                parameter_id,
                self.function_count,
            );

        self.variable_count += 1;

        var
    }

    /// Finds a variable
    fn resolve_symbol(&self, name: &String) -> Option<Symbol> {
        if name.contains("::") {
            let split = name.split("::").collect::<Vec<&str>>();

            let module = split[0];
            let name = split[1];

            let var = self
                .symbol_scope
                .get(module)
                .unwrap()
                .borrow()
                .resolve(name.to_string());

            if let Some(var) = var {
                return Option::from(var);
            }
        }

        self.symbol_scope
            .get(&self.location)
            .unwrap()
            .borrow()
            .resolve(name.to_string())
    }

    /// Compiles a deeper [Block] adding curly braces
    fn compile_block(
        &mut self,
        block: Block,
        _anonymous: bool,
    ) -> Result<Types, CompilerException> {
        let mut block_type: Types = Types::Void;
        self.enter_symbol_scope();

        self.add_to_current_function("{".to_string());

        let mut index = 0;
        for statement in block.statements.clone() {
            index += 1;

            let err = {
                if let Statement::Expression(ref exp) = statement {
                    if index != block.statements.len() {
                        // On their own these expressions dont provide side affects, so we don't
                        // want them to be compiled as they are useless
                        match *exp.expression {
                            Expression::Integer(_) => continue,
                            Expression::String(_) => continue,
                            Expression::Array(_) => continue,
                            Expression::Boolean(_) => continue,
                            Expression::Float(_) => continue,
                            Expression::Identifier(_) => continue,
                            _ => {}
                        };
                    }

                    if index == block.statements.len()
                        && !matches!(*exp.expression, Expression::AssignIndex(_))
                    {
                        self.add_to_current_function(".RETURN { ".to_string());
                    }

                    let result = self.compile_statement(statement.clone());

                    // Find first "return" as that is the only way to return
                    if let Statement::Return(_) = statement.clone() {
                        if let Ok(_type) = &result {
                            block_type = _type.clone();
                        }
                    }

                    // Or if its the last expression
                    if index == block.statements.len() {
                        if let Ok(_type) = &result {
                            block_type = _type.clone();
                        }

                        if !matches!(*exp.expression, Expression::AssignIndex(_)) {
                            self.add_to_current_function("};".to_string());
                        }
                    }

                    result
                } else {
                    let result = self.compile_statement(statement.clone());

                    // Find first "return" as that is the only way to return
                    if let Statement::Return(_) = statement.clone() {
                        if let Ok(_type) = &result {
                            block_type = _type.clone();
                        }
                    }

                    result
                }
            };

            #[allow(clippy::single_match)]
            match &err {
                Err(_exception) => {
                    return err;
                }
                _ => (),
            }
        }

        self.add_to_current_function("}".to_string());

        self.exit_symbol_scope();

        Ok(block_type)
    }

    /// Compiles the [Statement] [Node](crate::parser::program::Node)
    ///
    /// # Example
    /// ```loop
    /// let mut compiler = Compiler::default();
    /// let exp = Expression::Integer(Integer { value: 10 });
    /// let stmt = Statement::Expression(exp);
    ///
    /// let result = compiler.compile_statement(stmt);
    /// ```
    fn compile_statement(&mut self, stmt: Statement) -> Result<Types, CompilerException> {
        match stmt {
            Statement::VariableDeclaration(var) => {
                compile_statement_variable_declaration(self, var)
            }
            Statement::ConstantDeclaration(con) => {
                compile_statement_constant_declaration(self, con)
            }
            Statement::Expression(expr) => self.compile_expression(*expr.expression),
            Statement::Block(block) => self.compile_block(block, false),
            Statement::VariableAssign(variable) => {
                compile_statement_variable_assign(self, variable)
            }
            Statement::Return(_return) => compile_return_statement(self, _return),
            Statement::Import(import) => compile_import_statement(self, import),
            Statement::Break(br) => compile_break_statement(self, br),
            Statement::Class(class) => compile_class_statement(self, class),
            Statement::Extend(extend) => compile_extend_statement(self, extend),
        }
    }

    /// Throws an [CompilerError](crate::exception::compiler_new::CompilerError;) and exists with code '1'.
    fn throw_exception(&self, message: String, extra_message: Option<String>) {
        let mut err = CompilerError {
            error_message: message,
            extra_message,
        };
        err.throw_exception();
    }
}
