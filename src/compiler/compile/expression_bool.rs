use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::object::boolean;
use crate::lib::object::Object;
use crate::parser::expression::boolean::Boolean;

pub fn compile_expression_boolean(compiler: &mut Compiler, bl: Boolean) -> Option<String> {
    let value = match bl.value {
        true => Object::Boolean(boolean::Boolean { value: true }),
        false => Object::Boolean(boolean::Boolean { value: false }),
    };

    if let Object::Boolean(boolean) = value {
        let ct = compiler.add_constant(Object::Boolean(boolean::Boolean {
            value: boolean.value,
        }));
        compiler.emit(OpCode::Constant, vec![ct]);
    }

    None
}
