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
            return compile_expression_extension_method(_compiler, call, _index.left, true);
        }
        _ => {}
    }

    None
}

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
    deeper: bool,
) -> Option<CompilerException> {
    let method = match *call.identifier.clone() {
        Expression::Identifier(identifier) => identifier.value,
        _ => String::from(""),
    };

    // Search for extension method based on type
    let method_id = match left.clone() {
        Expression::Integer(integer) => {
            let extension = integer.find_extension(method.as_str());

            if extension.is_some() {
                compiler.last_extension_type = Option::from(extension.clone().unwrap().1);

                Some(extension.unwrap().0)
            } else {
                None
            }
        }
        Expression::String(string) => {
            let extension = string.find_extension(method.as_str());

            if extension.is_some() {
                compiler.last_extension_type = Option::from(extension.clone().unwrap().1);

                Some(extension.unwrap().0)
            } else {
                None
            }
        }
        Expression::Index(index) => {
            compile_expression_index(compiler, *index);

            let last_extension_type = compiler.last_extension_type.clone().unwrap();

            return compile_expression_extension_method(compiler, call, last_extension_type, false);
        }
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

    if deeper {
        compiler.compile_expression(left);
    }

    compiler.emit(
        OpCode::CallExtension,
        vec![method_id.unwrap() as u32, param_len as u32],
    );

    None
}
