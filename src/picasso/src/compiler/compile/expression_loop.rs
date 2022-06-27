use crate::compiler::Compiler;
use crate::exception::compiler::CompilerException;
use crate::parser::expression::index::Index;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use crate::parser::expression::Expression;
use crate::parser::types::{BaseTypes, Types};

use super::expression_index::compile_expression_index;

/// Compiles (/transpiles) the "while" loop of Loop
///
/// Take this example in Loop
/// ```loop
/// x := 0
/// for(x < 10) {
///     x = x + 1
/// }
/// println(x)
/// ```
///
/// Will translate to this D code (excluding imports & main declaration)
/// ```d
/// x := 0;
/// while(x < 10) {
///     x = x + 1;
/// }
/// writeln(x);
/// ```
pub fn compile_loop_expression(
    compiler: &mut Compiler,
    lp: Loop,
) -> Result<Types, CompilerException> {
    compiler.enter_symbol_scope();

    // Condition
    compiler.add_to_current_function(".WHILE CONDITION {".to_string());
    let result = compiler.compile_expression(*lp.condition);
    if let Err(exception) = result {
        return Err(exception);
    }

    compiler.add_to_current_function("} THEN ".to_string());

    // Body
    let result = compiler.compile_block(lp.body, false);
    compiler.add_to_current_function(";".to_string());

    result
}

/// Compiles (/transpiles) the "iterator" loop of Loop
///
/// Take this example in Loop
/// ```loop
/// x := 0
/// for(var n = 0..10) {
///     x = x + 1
/// }
/// println(x)
/// ```
///
/// Will translate to this D code (excluding imports & main declaration)
/// ```d
/// auto x = 0;
/// auto n = 0;
/// while(n < 10) {
///     x = x + 1;
///     n = n + 1;
/// }
/// writeln(x);
/// ```
pub fn compile_loop_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopIterator,
) -> Result<Types, CompilerException> {
    compiler.enter_symbol_scope();
    // Define the identifier variable, with the starting integer
    let var = compiler.define_symbol(lp.identifier.value, Types::Basic(BaseTypes::Integer), -1);

    compiler.add_to_current_function(format!(
        ".STORE {} {{.CONSTANT INT {};}};",
        var.index, lp.from
    ));

    compiler.add_to_current_function(format!(
        ".WHILE CONDITION {{ .GREATERTHAN {{ .CONSTANT INT {}; .LOAD VARIABLE {}; }}; }} THEN {{",
        lp.till, var.index
    ));

    // Compile the body that is executed
    let result = compiler.compile_loop_block(lp.body)?;

    compiler.exit_symbol_scope();

    // Increase it
    compiler.add_to_current_function(format!(
        ".STORE {} {{ .ADD {{.LOAD VARIABLE {};.CONSTANT INT 1;}};}};",
        var.index, var.index
    ));
    compiler.add_to_current_function("};".to_string());

    Ok(result)
}

/// Compiles (/transpiles) the "while" loop of Loop
///
/// Take this example in Loop
/// ```loop
/// array := [10, 20, 30]
/// for(var value in array) {
///     println(value)
/// }
/// ```
///
/// Will translate to this D code (excluding imports & main declaration)
/// ```d
/// int[] var_array_0 = [10, 20, 30];
/// foreach (var_value_1; var_array_0)/// {
///     writeln(var_value_1);
/// }
/// ```
pub fn compile_loop_array_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopArrayIterator,
) -> Result<Types, CompilerException> {
    compiler.enter_symbol_scope();

    // Define the identifier variable, with the starting value of the array
    let var = compiler.define_symbol(lp.identifier.value, Types::Basic(BaseTypes::Integer), -1);
    let index = compiler.define_symbol("INDEX_D".to_string(), Types::Basic(BaseTypes::Integer), -1);

    compiler.add_to_current_function(format!(".STORE {} {{ .CONSTANT INT 0; }};", index.index));
    compiler.add_to_current_function(format!(".STORE {} {{ ", var.index));

    // TODO: Get result and set it as type of 'var'
    let _ = compile_expression_index(
        compiler,
        Index {
            left: *lp.array.clone(),
            index: Expression::Integer(Integer { value: 0 }),
        },
    );

    compiler.add_to_current_function("}; .WHILE CONDITION { .GREATERTHAN { .LENGTH {".to_string());
    compiler.compile_expression(*lp.array.clone())?;
    compiler.add_to_current_function(format!(
        " }}; .LOAD VARIABLE {}; }}; }} THEN {{",
        index.index
    ));

    // Compile body and then increase the index
    let result = compiler.compile_loop_block(lp.body)?;

    compiler.add_to_current_function(format!(
        ".STORE {} {{ .ADD {{.LOAD VARIABLE {};.CONSTANT INT 1;}};}};",
        index.index, index.index
    ));
    compiler.add_to_current_function(format!(".STORE {} {{ .INDEX {{", var.index));

    compiler.compile_expression(*lp.array)?;

    compiler.add_to_current_function(format!("}} {{ .LOAD VARIABLE {}; }}; }};", index.index));

    compiler.add_to_current_function("};".to_string());

    Ok(result)
}
