use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression::function::Call;
use crate::parser::types::Types;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    compiler.add_to_current_function(".CALL {".to_string());

    let result = compiler.compile_expression(*call.identifier);

    #[allow(clippy::single_match)]
    let func_signature = match &result {
        CompilerResult::Exception(_exception) => return result,
        CompilerResult::Success(_type) => {
            if let Types::Function(func) = _type {
                func
            } else {
                return CompilerResult::Exception(CompilerException::CallingNonFunction(
                    _type.transpile(),
                ));
            }
        }
        _ => return CompilerResult::Exception(CompilerException::Unknown),
    };

    compiler.add_to_current_function(String::from("} {"));

    for parameter in call.parameters {
        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }
    }

    compiler.add_to_current_function(String::from("};"));

    CompilerResult::Success(*func_signature.return_type.clone())
}
