use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::expression::function::Call;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;

pub fn compile_expression_index(
    _compiler: &mut Compiler,
    _index: Index,
) -> Option<CompilerException> {
    match _index.right.clone() {
        Expression::Call(call) => {
            return compile_expression_extension_method(_compiler, call, _index.left);
        }
        _ => {}
    }

    None
}

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
) -> Option<CompilerException> {
    let method = match *call.identifier.clone() {
        Expression::Identifier(identifier) => identifier.value,
        _ => String::from(""),
    };

    // Search for extension method based on type
    let method_id = match left.clone() {
        Expression::Integer(integer) => integer.find_extension(method.as_str()),
        _ => return Some(CompilerException::UnknownExtensionMethod(method)),
    };

    if method_id.is_none() {
        return Some(CompilerException::UnknownExtensionMethod(method));
    }

    for parameter in call.parameters.clone() {
        let err = compiler.compile_expression(parameter);
        if err.is_some() {
            return err;
        }
    }

    let param_len = call.parameters.len();

    compiler.compile_expression(left);

    compiler.emit(
        OpCode::CallExtension,
        vec![method_id.unwrap() as u32, param_len as u32],
    );

    None
}
