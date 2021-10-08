use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::object::boolean;
use crate::object::{Object, FALSE, TRUE};
use crate::parser::expression::boolean::Boolean;

pub fn compile_expression_boolean(compiler: &mut Compiler, bl: Boolean) -> Option<String> {
    let value = match bl.value {
        true => &TRUE,
        false => &FALSE,
    };

    if let Object::Boolean(boolean) = value {
        let ct = compiler.add_constant(Object::Boolean(boolean::Boolean {
            value: boolean.value,
        }));
        compiler.emit(OpCode::Constant, vec![ct]);
    }

    None
}
