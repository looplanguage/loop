use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::function::Call;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    let result = compiler.compile_expression(*call.identifier.clone());

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    compiler.add_to_current_function(String::from("("));

    let mut current = 0;
    for parameter in call.parameters.clone() {
        current += 1;

        let result = compiler.compile_expression(parameter);

        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }

        if call.parameters.len() > 1 && current != call.parameters.len() {
            compiler.add_to_current_function(String::from(","));
        }
    }


    compiler.add_to_current_function(String::from(")"));

    CompilerResult::Success
}
