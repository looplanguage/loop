use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::suffix::Suffix;
use crate::parser::expression::Expression;

pub fn compile_expression_suffix(_compiler: &mut Compiler, _suffix: Suffix) -> Option<String> {
    let right = _suffix.right.clone();

    if _suffix.operator.as_str() == "<" {
        _compiler.compile_expression(_suffix.right);
        _compiler.compile_expression(_suffix.left);
    } else {
        _compiler.compile_expression(_suffix.left);
        _compiler.compile_expression(_suffix.right);
    }

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
            match right {
                Expression::Integer(integer) => {
                    if integer.value == 0 {
                        return Some("can not divide by 0".to_string());
                    }
                }
                Expression::Float(float) => {
                    if float.value == 0.0 {
                        return Some("can not divide by 0".to_string());
                    }
                }
                _ => {}
            }

            _compiler.emit(OpCode::Divide, vec![]);
        }
        "%" => {
            _compiler.emit(OpCode::Modulo, vec![]);
        }
        "==" => {
            _compiler.emit(OpCode::Equals, vec![]);
        }
        "!=" => {
            _compiler.emit(OpCode::NotEquals, vec![]);
        }
        ">" | "<" => {
            _compiler.emit(OpCode::GreaterThan, vec![]);
        }
        _ => {
            return Some(format!("unknown operator. got=\"{}\"", _suffix.operator));
        }
    }

    None
}
