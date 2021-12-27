use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::function::Call;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> CompilerResult {
    let result = compiler.compile_expression(*call.identifier.clone());

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    for parameter in call.parameters.clone() {
        let result = compiler.compile_expression(parameter);
        #[allow(clippy::single_match)]
        match &result {
            CompilerResult::Exception(_exception) => return result,
            _ => (),
        }
    }

    let param_len = call.parameters.len();

    compiler.emit(OpCode::Call, vec![param_len as u32]);

    CompilerResult::Success
}