use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::conditional::Conditional;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::types::Types;

pub fn compile_expression_conditional(
    compiler: &mut Compiler,
    conditional: Conditional,
    is_statement: bool,
) -> CompilerResult {
    // User needs to enable optimization, for Loop to optimize code.
    // Right now only does hardcoded "true" and "false" values
    // TODO: Is is commented because it does not work yet
    // if CONFIG.enable_optimize {
    //     let result = compile_expression_conditional_optimize(compiler, conditional.clone());
    //     // "true" means that optimization is successful.
    //     if result {
    //         return CompilerResult::Optimize;
    //     }
    // }

    let mut if_type: Types = Types::Void;
    let signature = if is_statement { "if (" } else { "() { if (" };

    compiler.add_to_current_function(signature.to_string());
    let result = compiler.compile_expression(*conditional.condition, false);
    compiler.add_to_current_function(")".to_string());

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        _ => (),
    }

    let result = compiler.compile_block(conditional.body);

    #[allow(clippy::single_match)]
    match &result {
        CompilerResult::Exception(_exception) => return result,
        CompilerResult::Success(if_type_result) => {
            if_type = if_type_result.clone();
        }
        _ => (),
    }

    if conditional.else_condition.is_some() {
        compiler.add_to_current_function(" else ".to_string());
    }

    if let Some(node) = conditional.else_condition.as_ref() {
        if let Node::Expression(exp) = node {
            compiler.add_to_current_function("{ ".to_string());
            compiler.compile_expression(exp.clone(), false);
            compiler.add_to_current_function("; }".to_string());
        }
        if let Node::Statement(stmt) = node {
            if let Statement::Block(block) = stmt.clone() {
                compiler.compile_block(block);
            }
        }
    }

    let signature = if is_statement { "" } else { "}()" };

    compiler.add_to_current_function(signature.to_string());

    CompilerResult::Success(if_type)
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
