use crate::compiler::compile::expression_null::compile_expression_null;
use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::config::CONFIG;
use crate::parser::expression::conditional::Conditional;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::statement::Statement;

pub fn compile_expression_conditional(
    compiler: &mut Compiler,
    conditional: Conditional,
) -> CompilerResult {
    // User needs to enable optimization, for Loop to optimize code.
    // Right now only does hardcoded "true" and "false" values
    // ToDo: Is is commented because it does not work yet
    // if CONFIG.enable_optimize {
    //     let result = compile_expression_conditional_optimize(compiler, conditional.clone());
    //     // "true" means that optimization is successful.
    //     if result {
    //         return CompilerResult::Optimize;
    //     }
    // }

    let mut result = compiler.compile_expression(*conditional.condition);

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    let position_false = compiler.emit(OpCode::JumpIfFalse, vec![0]);

    result = compiler.compile_block(conditional.body);

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    compiler.remove_last(OpCode::Pop);

    // TODO: Improve this somehow
    if !compiler.last_is(OpCode::Return)
        && !compiler.last_is(OpCode::Constant)
        && !compiler.last_is(OpCode::GetVar)
        && !compiler.last_is(OpCode::Multiply)
        && !compiler.last_is(OpCode::Add)
        && !compiler.last_is(OpCode::Minus)
        && !compiler.last_is(OpCode::Divide)
        && !compiler.last_is(OpCode::Modulo)
    {
        compiler.emit(OpCode::Constant, vec![0]); // adds NULL to the stack
    }

    let jump_to_end = compiler.emit(OpCode::Jump, vec![0]);

    if conditional.else_condition.is_none() {
        let len = compiler.scope().instructions.len() as u32;
        compiler.change_operand(position_false as u32, vec![len]);
    }

    compile_expression_null(compiler);

    if conditional.else_condition.is_some() {
        let len = compiler.scope().instructions.len() as u32;
        compiler.change_operand(position_false as u32, vec![len]);
    }

    if let Some(node) = conditional.else_condition.as_ref() {
        if let Node::Expression(exp) = node {
            compiler.compile_expression(exp.clone());
        }
        if let Node::Statement(stmt) = node {
            if let Statement::Block(block) = stmt.clone() {
                compiler.compile_block(block);

                compiler.remove_last(OpCode::Pop);
            }
        }
    }

    let len = compiler.scope().instructions.len() as u32;
    compiler.change_operand(jump_to_end as u32, vec![len]);

    CompilerResult::Success
}

// ToDo: This does not work yet. Hence it is commented
// fn compile_expression_conditional_optimize(
//     compiler: &mut Compiler,
//     conditional: Conditional,
// ) -> bool {
//     #[allow(clippy::single_match)]
//     match conditional.condition.as_ref() {
//         Expression::Boolean(boolean) => {
//             // Does does compile if-expression
//             if !boolean.value {
//                 compiler.remove_last(OpCode::Pop);
//
//                 if let Some(node) = conditional.else_condition.as_ref() {
//                     println!("else");
//                     if let Node::Expression(exp) = node {
//                         compiler.compile_expression(exp.clone());
//                     }
//                     if let Node::Statement(stmt) = node {
//                         if let Statement::Block(block) = stmt.clone() {
//                             compiler.compile_block(block);
//
//                             compiler.remove_last(OpCode::Pop);
//                         }
//                     }
//                 }
//                 return true;
//             }
//             // Only compiles the block of the if-expression
//             else {
//                 let result = compiler.compile_block(conditional.body);
//
//                 #[allow(clippy::single_match)]
//                 match &result {
//                     CompilerResult::Exception(_exception) => return false,
//                     _ => (),
//                 }
//
//                 compiler.remove_last(OpCode::Pop);
//             }
//             return true;
//         }
//         _ => (),
//     }
//
//     false
// }
