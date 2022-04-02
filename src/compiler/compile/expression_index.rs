use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::extension_method::lookup;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::function::Call;
use crate::parser::expression::index::Index;
use crate::parser::expression::Expression;

pub fn compile_expression_index(_compiler: &mut Compiler, _index: Index) -> CompilerResult {
    // Change to a match when indexing with [] (eg array[0])

    #[allow(clippy::single_match)]
    match _index.index.clone() {
        Expression::Call(call) => {
            compile_expression_extension_method(_compiler, call, _index.left, true)
        }
        _ => compile_expression_index_internal(_compiler, _index.left, _index.index),
    }
}

pub fn compile_expression_assign_index(
    compiler: &mut Compiler,
    assign: AssignIndex,
) -> CompilerResult {
    compiler.compile_expression(assign.left);

    compiler.add_to_current_function("[".to_string());
    compiler.compile_expression(assign.index);
    compiler.add_to_current_function("] = ".to_string());
    compiler.compile_expression(assign.value);

    CompilerResult::Success
}

fn compile_expression_index_internal(
    compiler: &mut Compiler,
    left: Expression,
    index: Expression,
) -> CompilerResult {
    compiler.compile_expression(left);
    compiler.add_to_current_function("[".to_string());
    compiler.compile_expression(index);
    compiler.add_to_current_function("]".to_string());

    compiler.emit(OpCode::Index, vec![]);

    CompilerResult::Success
}

pub fn compile_expression_extension_method(
    compiler: &mut Compiler,
    call: Call,
    left: Expression,
    deeper: bool,
) -> CompilerResult {
    let method = match *call.identifier.clone() {
        Expression::Identifier(identifier) => identifier.value,
        _ => String::from(""),
    };

    // Search extension id
    let method_id = lookup(method.as_str());

    if method_id.is_none() {
        return CompilerResult::Exception(CompilerException::UnknownExtensionMethod(method));
    }

    /*
        extension!(to_string),
        extension!(to_int), // 1
        extension!(add), // 2
        extension!(remove), // 3
        extension!(slice), // 4
        extension!(length), // 5
     */

    let param_len = call.parameters.len();

    if deeper {
        compiler.compile_expression(left.clone());
    }

    match method_id.unwrap() {
        2 => {
            compiler.add_to_current_function(" ~= ".to_string());

            let mut index = 0;

            if call.parameters.len() > 1 {
                compiler.add_to_current_function("[".to_string());
            }

            for parameter in call.parameters.clone() {
                let result = compiler.compile_expression(parameter);

                #[allow(clippy::single_match)]
                match &result {
                    CompilerResult::Exception(_exception) => return result,
                    _ => (),
                }

                index += 1;

                if call.parameters.len() > 1 && call.parameters.len() != index {
                    compiler.add_to_current_function(", ".to_string());
                }
            }

            if call.parameters.len() > 1 {
                compiler.add_to_current_function("]".to_string());
            }
        },
        3 => {
            compiler.add_to_current_function(" = ".to_string());

            if deeper {
                compiler.compile_expression(left);
            }

            compiler.add_to_current_function(".remove(".to_string());

            let mut index = 0;
            for parameter in call.parameters.clone() {
                let result = compiler.compile_expression(parameter);

                #[allow(clippy::single_match)]
                match &result {
                    CompilerResult::Exception(_exception) => return result,
                    _ => (),
                }

                index += 1;

                if call.parameters.len() > 1 && call.parameters.len() != index {
                    compiler.add_to_current_function(", ".to_string());
                }
            }

            compiler.add_to_current_function(")".to_string());
        },
        4 => {
            compiler.add_to_current_function("[".to_string());

            let start = call.parameters[0].clone();
            let end = call.parameters[1].clone();

            compiler.compile_expression(start);

            compiler.add_to_current_function("..".to_string());

            compiler.compile_expression(end);

            compiler.add_to_current_function("]".to_string());
        }
        5 => {
            compiler.add_to_current_function(".length".to_string());
        }
        _ => {}
    }

    compiler.emit(
        OpCode::CallExtension,
        vec![method_id.unwrap() as u32, param_len as u32],
    );

    CompilerResult::Success
}
