use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::suffix::Suffix;

pub fn compile_expression_suffix(_compiler: &mut Compiler, _suffix: Suffix) -> Option<String> {
    _compiler.compile_expression(_suffix.left);
    _compiler.compile_expression(_suffix.right);

    match _suffix.operator.as_str() {
        "+" => {
            _compiler.emit(OpCode::Add, vec![]);
        }
        "-" => {
            _compiler.emit(OpCode::Minus, vec![]);
        }
        "*" => {
            _compiler.emit(OpCode::Multiply, vec![]);
        }
        "/" => {
            _compiler.emit(OpCode::Divide, vec![]);
        }
        "%" => {
            _compiler.emit(OpCode::Modulo, vec![]);
        }
        _ => {
            panic!(format!("unknown operator. got={}", _suffix.operator))
        }
    }

    None
}
