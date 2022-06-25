use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::suffix::Suffix;
use crate::parser::types::{BaseTypes, Types};

pub fn compile_expression_suffix(
    _compiler: &mut Compiler,
    _suffix: Suffix,
) -> Result<Types, CompilerException> {
    match _suffix.operator.as_str() {
        "^" => {
            _compiler.add_to_current_function(".POWER {".to_string());
        }
        "and" => {
            _compiler.add_to_current_function(".AND {".to_string());
        }
        "or" => {
            _compiler.add_to_current_function(".OR {".to_string());
        }
        "+" => {
            _compiler.add_to_current_function(".ADD {".to_string());
        }
        "-" => {
            _compiler.add_to_current_function(".SUBTRACT {".to_string());
        }
        "*" => {
            _compiler.add_to_current_function(".MULTIPLY {".to_string());
        }
        "/" => {
            _compiler.add_to_current_function(".DIVIDE {".to_string());
        }
        ">" | "<" => {
            _compiler.add_to_current_function(".GREATERTHAN {".to_string());
        }
        "==" => {
            _compiler.add_to_current_function(".EQUALS {".to_string());
        }
        "!=" => {
            _compiler.add_to_current_function(".NOTEQUALS {".to_string());
        }
        "%" => {
            _compiler.add_to_current_function(".MODULO {".to_string());
        }
        _ => {
            _compiler.add_to_current_function("UNKNOWN_OPERATOR".to_string());
        }
    }

    if _suffix.operator == "<" {
        _compiler.compile_expression(_suffix.right)?;
        _compiler.compile_expression(_suffix.left)?;
    } else {
        _compiler.compile_expression(_suffix.left)?;
        _compiler.compile_expression(_suffix.right)?;
    }
    _compiler.add_to_current_function("};".to_string());

    // Suffix expressions are currently only for integers
    Ok(Types::Basic(BaseTypes::Integer))
}
