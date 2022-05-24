//! Responsible for transpiling Loop to D
mod compile;
mod modifiers;
mod test;
mod variable_table;

use crate::compiler::compile::expression_array::compile_expression_array;
use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_call::compile_expression_call;
use crate::compiler::compile::expression_conditional::compile_expression_conditional;
use crate::compiler::compile::expression_float::compile_expression_float;
use crate::compiler::compile::expression_function::{compile_expression_function, Function};
use crate::compiler::compile::expression_hashmap::compile_expression_hashmap;
use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::compile::expression_index::{
    compile_expression_assign_index, compile_expression_index,
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
use crate::compiler::compile::statement_export::compile_export_statement;
use crate::compiler::compile::statement_import::compile_import_statement;
use crate::compiler::compile::statement_return::compile_return_statement;
use crate::compiler::compile::statement_variable_assign::compile_statement_variable_assign;
use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::modifiers::Modifiers;
use crate::compiler::variable_table::{
    build_deeper_variable_scope, build_variable_scope, Variable, VariableScope,
};
use crate::exception::compiler::CompilerException;
use crate::exception::compiler_new::CompilerError;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::block::Block;
use crate::parser::statement::Statement;
use crate::parser::types::Types;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Instance of CompilerResult which contains information on how the compiler handled input
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum CompilerResult {
    // Success can return an optional type if the result was an expression
    Success(Types),
    Optimize,
    Exception(CompilerException),
}

/// The result of the transpiler, which will be passed to the D compiler [crate::util::execute_code]
pub struct DCode {
    pub imports: Vec<String>,
    pub functions: HashMap<String, Function>,
}

impl DCode {
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
    pub variable_scope: Rc<RefCell<VariableScope>>,
    pub variable_count: u32,
    pub last_extension_type: Option<Expression>,
    pub location: String,
    pub export_name: String,
    pub prev_location: String,
    pub breaks: Vec<u32>,

    pub imports: Vec<String>,
    pub functions: HashMap<String, Function>,
    pub function_stack: Vec<String>,
    pub function_count: i32,
    pub current_function: String,
    // Specifies whether or not compiling should add code
    pub dry: bool,
}

#[derive(Clone)]
pub struct CompilerState {
    pub variable_scope: Rc<RefCell<VariableScope>>,
    pub variable_count: u32,
    pub function_count: i32,
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            scope_index: 0,
            variable_count: 0,
            variable_scope: Rc::new(RefCell::new(build_variable_scope())),
            last_extension_type: None,
            location: String::new(),
            export_name: String::new(),
            prev_location: String::new(),
            breaks: Vec::new(),

            imports: Vec::new(),
            functions: HashMap::new(),
            function_stack: Vec::new(),
            function_count: 0,
            current_function: String::from("main"),
            dry: false,
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
    pub fn compile(&mut self, program: Program) -> Result<DCode, CompilerException> {
        // Insert main function
        if self.location.is_empty() {
            let main_function = Function {
                name: "main".to_string(),
                code: String::from(""),
                parameters: Vec::new(),
                return_type: Types::Void,
            };

            self.functions.insert(String::from("main"), main_function);
        }

        let mut index = 0;
        let length = program.statements.len();
        for statement in program.statements {
            index += 1;

            let mut is_expression = false;
            if index == length {
                if let Statement::Expression(_) = statement.clone() {
                    is_expression = true;
                }
            }

            let err = self.compile_statement(statement, is_expression);

            #[allow(clippy::single_match)]
            match err {
                CompilerResult::Exception(exception) => return Result::Err(exception),
                _ => (),
            }
        }

        Result::Ok(self.get_d_code())
    }

    pub fn get_compound_type(&self, name: &String) -> Option<Types> {
        println!("Called!");
        let class =
            self.variable_scope
                .borrow_mut()
                .resolve(format!("{}{}", self.location, name.clone()));

        println!("Name: {:?}", name);
        println!("Found: {:?}", class);

        if let Some(class) = class {
            if let Types::Compound(name, values) = class._type {
                // Instantiate the class using a constant
                let mut cloned_values = values.clone();

                let mut index = 0;
                for value in &mut *cloned_values {
                    value.1 .0 = index;

                    index += 1;
                }
                println!("Some!");

                return Some(Types::Compound(name.clone(), cloned_values));
            }

            if let Types::Auto = class._type {
                return Some(Types::Auto);
            }
        }

        println!("Bad!");
        None
    }

    pub fn default_with_state(compiler_state: CompilerState) -> Compiler {
        Compiler {
            function_count: compiler_state.function_count,
            variable_count: compiler_state.variable_count,
            variable_scope: compiler_state.variable_scope,
            ..Compiler::default()
        }
    }

    pub fn get_compiler_state(&self) -> CompilerState {
        CompilerState {
            function_count: self.function_count,
            variable_count: self.variable_count,
            variable_scope: self.variable_scope.clone(),
        }
    }

    /// Adds code to the current function compilation scope
    pub fn add_to_current_function(&mut self, code: String) {
        if !self.dry {
            let func = self.functions.get_mut(&*self.current_function);

            func.unwrap().code.push_str(code.as_str());
        }
    }

    /// Allows replacing context
    pub fn replace_at_current_function(&mut self, replace: String, with: String) {
        if !self.dry {
            let func = self.functions.get_mut(&*self.current_function);

            let unwrapped = func.unwrap();
            let replaced = unwrapped.code.replace(replace.as_str(), &*with);

            unwrapped.code = replaced;
        }
    }

    /// Enter a deeper variable scope
    pub fn enter_variable_scope(&mut self) {
        let scope = build_deeper_variable_scope(Option::from(self.variable_scope.clone()));
        self.scope_index += 1;
        self.variable_scope = Rc::new(RefCell::new(scope));
    }

    /// Exit a variable scope and go one shallower
    pub fn exit_variable_scope(&mut self) {
        let outer = self.variable_scope.as_ref().borrow_mut().outer.clone();
        self.scope_index -= 1;
        self.variable_scope = outer.unwrap();
    }

    pub fn get_d_code(&self) -> DCode {
        DCode {
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
    fn compile_expression(&mut self, expr: Expression) -> CompilerResult {
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
        }
    }

    /// Compiles a loop [Block], this differs from [Compiler::compile_block] in that it wont add curly braces
    fn compile_loop_block(&mut self, block: Block) -> CompilerResult {
        let mut return_type = Types::Void;
        self.enter_variable_scope();

        let mut index = 0;
        for statement in block.statements.clone() {
            index += 1;

            let err = self.compile_statement(statement.clone(), false);

            // If its either a return statement, or the last statement is an expression than that is the return type of this block
            if let Statement::Return(_) = statement {
                if let CompilerResult::Success(_type) = err.clone() {
                    return_type = _type;
                }
            }

            if index == block.statements.len() {
                if let Statement::Expression(_) = statement {
                    if let CompilerResult::Success(_type) = err.clone() {
                        return_type = _type;
                    }
                }
            }

            #[allow(clippy::single_match)]
            match &err {
                CompilerResult::Exception(_exception) => {
                    return CompilerResult::Exception(_exception.clone())
                }
                _ => (),
            }
        }

        self.exit_variable_scope();

        CompilerResult::Success(return_type)
    }

    /// Defines a new variable and increases the amount of variables that exist
    fn define_variable(&mut self, name: String, var_type: Types, parameter_id: i32) -> Variable {
        let var = self.variable_scope.borrow_mut().define(
            self.variable_count,
            format!("{}{}", self.location, name),
            var_type,
            Modifiers::default(),
            parameter_id,
            self.function_count,
        );

        self.variable_count += 1;

        var
    }

    /// Compiles a deeper [Block] adding curly braces
    fn compile_block(&mut self, block: Block, anonymous: bool) -> CompilerResult {
        let mut block_type: Types = Types::Void;
        self.enter_variable_scope();

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

                    if index == block.statements.len() && anonymous {
                        self.add_to_current_function(".RETURN { ".to_string())
                    }

                    let result = self.compile_statement(statement.clone(), false);

                    // Find first "return" as that is the only way to return
                    if let Statement::Return(_) = statement.clone() {
                        if let CompilerResult::Success(_type) = &result {
                            block_type = _type.clone();
                        }
                    }

                    // Or if its the last expression
                    if index == block.statements.len() {
                        if let CompilerResult::Success(_type) = &result {
                            block_type = _type.clone();
                        }

                        if anonymous {
                            self.add_to_current_function("};".to_string());
                        }
                    }

                    result
                } else {
                    let result = self.compile_statement(statement.clone(), false);

                    // Find first "return" as that is the only way to return
                    if let Statement::Return(_) = statement.clone() {
                        if let CompilerResult::Success(_type) = &result {
                            block_type = _type.clone();
                        }
                    }

                    result
                }
            };

            #[allow(clippy::single_match)]
            match &err {
                CompilerResult::Exception(_exception) => {
                    return err;
                }
                _ => (),
            }
        }

        self.add_to_current_function("}".to_string());

        self.exit_variable_scope();

        CompilerResult::Success(block_type)
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
    fn compile_statement(&mut self, stmt: Statement, no_semicolon: bool) -> CompilerResult {
        let mut expression_statement = false;

        let result = match stmt.clone() {
            Statement::VariableDeclaration(var) => {
                compile_statement_variable_declaration(self, var)
            }
            Statement::ConstantDeclaration(con) => {
                compile_statement_constant_declaration(self, con)
            }
            Statement::Expression(expr) => {
                expression_statement = true;
                self.compile_expression(*expr.expression)
            }
            Statement::Block(block) => self.compile_block(block, false),
            Statement::VariableAssign(variable) => {
                compile_statement_variable_assign(self, variable)
            }
            Statement::Return(_return) => compile_return_statement(self, _return),
            Statement::Import(import) => compile_import_statement(self, import),
            Statement::Export(export) => compile_export_statement(self, export),
            Statement::Break(br) => compile_break_statement(self, br),
            Statement::Class(class) => compile_class_statement(self, class),
        };

        let add_semicolon = match stmt {
            Statement::VariableDeclaration(_) => true,
            Statement::ConstantDeclaration(_) => true,
            Statement::Expression(expr) => match *expr.expression {
                Expression::Conditional(_) => !expression_statement,
                Expression::Loop(_) => true,
                Expression::LoopIterator(_) => false,
                Expression::LoopArrayIterator(_) => false,
                Expression::Function(func) => func.name.is_empty(),
                _ => true,
            },
            Statement::Block(_) => false,
            Statement::VariableAssign(_) => true,
            Statement::Return(_) => true,
            Statement::Import(_) => false,
            Statement::Export(_) => true,
            Statement::Break(_) => true,
            Statement::Class(_) => true,
        };

        if add_semicolon && !no_semicolon {
            //self.add_to_current_function(";".to_string());
        }

        result
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
