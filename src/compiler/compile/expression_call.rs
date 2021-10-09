use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::expression::function::Call;

pub fn compile_expression_call(compiler: &mut Compiler, call: Call) -> Option<String> {
    for parameter in call.parameters.clone() {
        let err = compiler.compile_expression(parameter);
        if err.is_some() {
            return err;
        }
    }

    compiler.compile_expression(*call.identifier);
    let param_len = call.parameters.len();

    println!("CALL COMPILED");

    compiler.emit(OpCode::Call, vec![param_len as u32]);

    None
}
