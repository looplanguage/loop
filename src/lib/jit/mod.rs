use crate::lib::object::Object;
use crate::parser::expression::function::Function;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::FloatValue;
use std::cell::RefCell;
use std::rc::Rc;

type DoubleFunc = unsafe extern "C" fn() -> u64;

#[allow(dead_code)]
pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) compiled_functions: Vec<Option<JitFunction<'ctx, DoubleFunc>>>,
}

// TODO: Document this quite a bit more, as this is a little complicated
impl<'ctx> CodeGen<'ctx> {
    #[allow(dead_code)]
    pub fn compile(&mut self, func: Function) -> Option<bool> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into()], false);
        let function = self.module.add_function("double", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        self.compile_statement(func.body.statements);

        self.compiled_functions
            .push(unsafe { self.execution_engine.get_function("double").ok() });

        println!("COMPILED!");

        None
    }

    fn compile_statement(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::VariableDeclaration(_) => {}
                Statement::Expression(_exp) => {
                    //self.compile_expression(*exp.expression);
                }
                Statement::Block(_) => {}
                Statement::VariableAssign(_) => {}
                Statement::Return(ret) => {
                    let return_val = { self.compile_expression_int(*ret.expression) };
                    self.builder.build_return(Some(&return_val));
                }
                Statement::Import(_) => {}
                Statement::Export(_) => {}
            };
        }
    }

    fn compile_expression_int(&mut self, expression: Expression) -> FloatValue<'ctx> {
        let f64_type = self.context.f64_type();

        match expression {
            Expression::Identifier(_) => {}
            Expression::Integer(int) => {
                return f64_type.const_float(int.value as f64);
            }
            Expression::Suffix(suffix) => {
                let lhs = self.compile_expression_int(suffix.left);
                let rhs = self.compile_expression_int(suffix.right);

                match suffix.operator.as_str() {
                    "+" => {
                        return self.builder.build_float_add(lhs, rhs, "add");
                    }
                    "/" => {
                        return self.builder.build_float_div(lhs, rhs, "divide");
                    }
                    _ => {}
                }
            }
            Expression::Boolean(_) => {}
            Expression::Function(_) => {}
            Expression::Conditional(_) => {}
            Expression::Null(_) => {}
            Expression::Call(_) => {}
            Expression::Float(_) => {}
            Expression::String(_) => {}
            Expression::Index(_) => {}
            Expression::Array(_) => {}
            Expression::AssignIndex(_) => {}
            Expression::Loop(_) => {}
            Expression::LoopIterator(_) => {}
            Expression::LoopArrayIterator(_) => {}
            Expression::Hashmap(_) => {}
        };

        f64_type.const_float(0 as f64)
    }

    #[allow(dead_code)]
    pub fn run(&self, id: i32, _params: Vec<Rc<RefCell<Object>>>) -> u64 {
        if let Some(compiled) = &self.compiled_functions[id as usize] {
            let _compiled_down_params: Vec<u64> = Vec::new();

            let returned = unsafe { compiled.call() };

            return returned;
        }

        0
    }
}
