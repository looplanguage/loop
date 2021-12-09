use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::suffix::Suffix;
use crate::parser::expression::Expression;

pub fn compile_expression_suffix(
    _compiler: &mut Compiler,
    _suffix: Suffix,
) -> Option<CompilerException> {
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
                        return Some(CompilerException::DivideByZero);
                    }
                }
                Expression::Float(float) => {
                    if float.value == 0.0 {
                        return Some(CompilerException::DivideByZero);
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
        "^" => {
            _compiler.emit(OpCode::Pow, vec![]);
        }
        "or" | "||"  => {
            _compiler.emit(OpCode::Or, vec![]);
        }
        "and" | "&&" => {
            _compiler.emit(OpCode::And, vec![]);
        }
        _ => {
            return Some(CompilerException::UnknownSuffixOperator(_suffix.operator));
        }
    }

    None
}
