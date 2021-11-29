use crate::lib::object::Object;
use crate::parser::expression::function::Function;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{FloatValue, FunctionValue, IntValue};
use inkwell::FloatPredicate;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type DoubleFunc = unsafe extern "C" fn(f64) -> f64;

#[allow(dead_code)]
pub struct CodeGen<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) execution_engine: ExecutionEngine<'ctx>,
    pub(crate) compiled_functions: HashMap<String, JitFunction<'ctx, DoubleFunc>>,
    pub(crate) parameters: Vec<String>,
}

// TODO: Document this quite a bit more, as this is a little complicated
impl<'ctx> CodeGen<'ctx> {
    pub fn get_function(&self, pointer: String) -> Option<&JitFunction<'ctx, DoubleFunc>> {
        self.compiled_functions.get(&*pointer)
    }

    pub fn compile(&mut self, func: Function, pointer: String) -> bool {
        let f64_type = self.context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into()], false);
        let function = self.module.add_function("double", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let ok = self.compile_statement(func.body.statements, func.parameters, function);

        if !ok {
            return false;
        }

        self.compiled_functions.insert(pointer.clone(), unsafe {
            self.execution_engine.get_function("double").ok().unwrap()
        });

        function.verify(true);

        true
    }

    fn compile_statement(
        &mut self,
        statements: Vec<Statement>,
        arguments: Vec<Identifier>,
        function: FunctionValue<'ctx>,
    ) -> bool {
        for statement in statements {
            match statement {
                Statement::VariableDeclaration(_) => return false,
                Statement::Expression(_exp) => {
                    self.compile_expression_float(*_exp.expression, &arguments, function);
                }
                Statement::Block(_) => return false,
                Statement::VariableAssign(_) => return false,
                Statement::Return(ret) => {
                    let return_val =
                        { self.compile_expression_float(*ret.expression, &arguments, function) };

                    if return_val.is_none() {
                        return false;
                    }

                    self.builder.build_return(Some(&return_val.unwrap()));
                }
                Statement::Import(_) => return false,
                Statement::Export(_) => return false,
            };
        }

        true
    }
    fn compile_expression_int(
        &mut self,
        expression: Expression,
        arguments: &[Identifier],
        function: FunctionValue<'ctx>,
    ) -> Option<IntValue<'ctx>> {
        let i64_type = self.context.i64_type();

        match expression {
            Expression::Suffix(suffix) => {
                let lhs = self.compile_expression_float(suffix.left, arguments, function)?;
                let rhs = self.compile_expression_float(suffix.right, arguments, function)?;

                match suffix.operator.as_str() {
                    "<" => Some(self
                        .builder
                        .build_float_compare(FloatPredicate::OLT, lhs, rhs, "")),
                    "==" => Some(self
                        .builder
                        .build_float_compare(FloatPredicate::OEQ, lhs, rhs, "")),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn compile_expression_float(
        &mut self,
        expression: Expression,
        arguments: &[Identifier],
        function: FunctionValue<'ctx>,
    ) -> Option<FloatValue<'ctx>> {
        let f64_type = self.context.f64_type();

        return match expression {
            Expression::Identifier(identifier) => {
                let parameter_id = arguments.iter().position(|x| x.value == identifier.value);

                if let Some(id) = parameter_id {
                    let param = function.get_nth_param(id as u32);

                    if let Some(param) = param {
                        let val = param.into_float_value();

                        return Some(val);
                    }
                }

                None
            }
            Expression::Integer(int) => {
                Some(f64_type.const_float(int.value as f64))
            }
            Expression::Suffix(suffix) => {
                let lhs = self.compile_expression_float(suffix.left, arguments, function)?;
                let rhs = self.compile_expression_float(suffix.right, arguments, function)?;

                match suffix.operator.as_str() {
                    "+" => Some(self.builder.build_float_add(lhs, rhs, "add")),
                    "/" => Some(self.builder.build_float_div(lhs, rhs, "divide")),
                    "-" => Some(self.builder.build_float_sub(lhs, rhs, "minus")),
                    "*" => Some(self.builder.build_float_mul(lhs, rhs, "multiply")),
                    _ => None,
                }
            }
            Expression::Boolean(_) => None,
            Expression::Function(_) => None,
            Expression::Conditional(conditional) => {
                let cond = self.compile_expression_int(
                    *conditional.condition.clone(),
                    arguments,
                    function,
                )?;

                // branches
                let then_b = self.context.append_basic_block(function, "then");
                let else_b = self.context.append_basic_block(function, "else");
                let cont_b = self.context.append_basic_block(function, "ifcont");

                self.builder.build_conditional_branch(cond, then_b, else_b);

                // then block
                self.builder.position_at_end(then_b);

                let then_exp = match conditional.body.statements[0].clone() {
                    Statement::Expression(exp) => *exp.expression,
                    _ => {
                        return None;
                    }
                };

                let then_val = self.compile_expression_float(then_exp, arguments, function)?;

                self.builder.build_unconditional_branch(cont_b);

                let then_b = self.builder.get_insert_block().unwrap();

                // else block
                self.builder.position_at_end(else_b);

                let else_exp = match *conditional.else_condition {
                    None => Expression::Integer(Integer { value: 0 }),
                    Some(stmt) => {
                        if let Node::Statement(Statement::Block(block)) = stmt {
                            if let Statement::Expression(exp) = block.statements[0].clone() {
                                *exp.expression
                            } else {
                                Expression::Integer(Integer { value: 0 })
                            }
                        } else {
                            Expression::Integer(Integer { value: 0 })
                        }
                    }
                };

                let else_val = self.compile_expression_float(else_exp, arguments, function)?;
                self.builder.build_unconditional_branch(cont_b);

                let else_b = self.builder.get_insert_block().unwrap();

                // merge
                self.builder.position_at_end(cont_b);

                let phi = self.builder.build_phi(self.context.f64_type(), "iftmp");

                phi.add_incoming(&[(&then_val, then_b), (&else_val, else_b)]);

                let return_val = phi.as_basic_value().into_float_value();

                self.builder.build_return(Some(&return_val));

                Some(return_val)
            }
            Expression::Null(_) => None,
            Expression::Call(_call) => {
                None
                /*
                //let param =
                //self.compile_expression_float(call.parameters[0].clone(), arguments, function);

                let arg = self.context.f64_type().const_float(0.0);

                let fn_type = self.module.get_function("double");

                self.builder
                    .build_call(fn_type.unwrap(), &[arg.into()], "call");

                 */
            }
            Expression::Float(_) => None,
            Expression::String(_) => None,
            Expression::Index(_) => None,
            Expression::Array(_) => None,
            Expression::AssignIndex(_) => None,
            Expression::Loop(_) => None,
            Expression::LoopIterator(_) => None,
            Expression::LoopArrayIterator(_) => None,
            Expression::Hashmap(_) => None,
        };

        None
    }

    #[allow(dead_code)]
    pub fn run(&mut self, ptr: String, _params: Vec<Rc<RefCell<Object>>>) -> f64 {
        if let Some(compiled) = self.get_function(ptr) {
            let mut _compiled_down_params: Vec<f64> = Vec::new();

            for _param in _params {
                if let Object::Integer(integer) = &*_param.as_ref().borrow() {
                    _compiled_down_params.push(integer.value as f64);
                }
            }

            let returned = unsafe { compiled.call(_compiled_down_params[0]) };

            println!("JIT RETURNED: {}", returned);

            return returned;
        }

        0 as f64
    }
}
