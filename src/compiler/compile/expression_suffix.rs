use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::suffix::Suffix;

pub fn compile_expression_suffix(_compiler: &mut Compiler, _suffix: Suffix) -> CompilerResult {
    _compiler.add_to_current_function("(".to_string());
    _compiler.compile_expression(_suffix.left);

    match _suffix.operator.as_str() {
        "^" => {
            _compiler.add_to_current_function("^^".to_string());
        }
        "and" => {
            _compiler.add_to_current_function("&&".to_string());
        }
        "or" => {
            _compiler.add_to_current_function("||".to_string());
        }
        _ => {
            _compiler.add_to_current_function(_suffix.operator);
        }
    }

    _compiler.compile_expression(_suffix.right);
    _compiler.add_to_current_function(")".to_string());

    /*let right = _suffix.right.clone();

    _compiler.add_to_current_function(_suffix.)

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
                        return CompilerResult::Exception(CompilerException::DivideByZero);
                    }
                }
                Expression::Float(float) => {
                    if float.value == 0.0 {
                        return CompilerResult::Exception(CompilerException::DivideByZero);
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
        "or" | "||" => {
            _compiler.emit(OpCode::Or, vec![]);
        }
        "and" | "&&" => {
            _compiler.emit(OpCode::And, vec![]);
        }
        _ => {
            return CompilerResult::Exception(CompilerException::UnknownSuffixOperator(
                _suffix.operator,
            ));
        }
    }

     */

    CompilerResult::Success
}
