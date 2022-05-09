use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::expression::function::Call;
use crate::parser::types::Types;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    compiler.dry = true;
    let result = compiler.compile_expression(*call.identifier.clone(), false);
    compiler.dry = false;

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

    println!("SIGNATURE: {:?}", func_signature);

    compiler.add_to_current_function(String::from(format!(".CALL {} {{", func_signature.reference)));

    let mut current = 0;
    for parameter in call.parameters.clone() {
        current += 1;

        let result = compiler.compile_expression(parameter, false);
        // Get proper type or cast to Variant if inferring type
        /*
        if *func_signature.return_type == Types::Auto {
            compiler.add_to_current_function(".to!Variant".to_string());
        }

         */

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }
    }

    compiler.add_to_current_function(String::from("};"));

    CompilerResult::Success(*func_signature.return_type.clone())
}
