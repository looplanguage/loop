use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::conditional::Conditional;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::types::Types;

pub fn compile_expression_conditional(
    compiler: &mut Compiler,
    conditional: Conditional,
) -> Result<Types, CompilerException> {
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

    compiler.add_to_current_function(".IF CONDITION { ".to_string());
    compiler.compile_expression(*conditional.condition)?;
    compiler.add_to_current_function(" } THEN ".to_string());

    let if_type = compiler.compile_block(conditional.body, true)?;

    compiler.add_to_current_function(" ELSE ".to_string());

    if let Some(node) = conditional.else_condition.as_ref() {
        if let Node::Expression(exp) = *node.clone() {
            compiler.add_to_current_function("{".to_string());
            compiler.compile_expression(exp)?;
            compiler.add_to_current_function("}".to_string());
        }
        if let Node::Statement(Statement::Block(block)) = *node.clone() {
            compiler.compile_block(block, true)?;
        }
    } else {
        compiler.add_to_current_function("{ }".to_string());
    }

    compiler.add_to_current_function(";".to_string());

    Ok(if_type)
}

// TODO: This does not work yet. Hence it is commented
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
