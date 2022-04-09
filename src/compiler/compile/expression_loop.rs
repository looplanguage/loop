use crate::compiler::{Compiler, CompilerResult};
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use crate::parser::types::{BaseTypes, Types};

/// Compiles (/transpiles) the "while" loop of Loop
///
/// Take this example in Loop
/// ```loop
/// var x = 0
/// for(x < 10) {
///     x = x + 1
/// }
/// println(x)
/// ```
///
/// Will translate to this D code (excluding imports & main declaration)
/// ```d
/// auto x = 0;
/// while(x < 10) {
///     x = x + 1;
/// }
/// writeln(x);
/// ```
pub fn compile_loop_expression(compiler: &mut Compiler, lp: Loop) -> CompilerResult {
    compiler.enter_variable_scope();

    // Condition
    compiler.add_to_current_function("() { while (".to_string());
    let result = compiler.compile_expression(*lp.condition, false);
    if let CompilerResult::Exception(exception) = result {
        return CompilerResult::Exception(exception);
    }

    compiler.add_to_current_function(")".to_string());

    // Body
    let result = compiler.compile_block(lp.body, true);

    compiler.add_to_current_function("}()".to_string());

    result
}

/// Compiles (/transpiles) the "iterator" loop of Loop
///
/// Take this example in Loop
/// ```loop
/// var x = 0
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
) -> CompilerResult {
    compiler.enter_variable_scope();
    // Define the identifier variable, with the starting integer
    let var = compiler.define_variable(lp.identifier.value, Types::Basic(BaseTypes::Integer));

    compiler.add_to_current_function(format!("int {} = {};", var.transpile(), lp.from));

    compiler.add_to_current_function(format!("while({} < {}) {{", var.transpile(), lp.till));

    // Compile the body that is executed
    let result = compiler.compile_loop_block(lp.body);

    compiler.exit_variable_scope();

    // Increase it
    compiler.add_to_current_function(format!("{} += 1; }}", var.transpile()));

    result
}

/// Compiles (/transpiles) the "while" loop of Loop
///
/// Take this example in Loop
/// ```loop
/// var array = [10, 20, 30]
/// for(var value in array) {
///     println(value)
/// }
/// ```
///
/// Will translate to this D code (excluding imports & main declaration)
/// ```d
/// auto var_array_0 = [10, 20, 30];
/// auto var__iterator_array_1 = var_array_0;
///
/// int var__iterator_index_3 = 0;
///
/// auto var_value_2 = var__iterator_array_1[0];
///
/// while(var__iterator_index_3 < var__iterator_array_1.length) {
///     writeln(var_value_2);var__iterator_index_3 += 1;
///
///     if(var__iterator_index_3 < var__iterator_array_1.length) {
///         var_value_2 = var__iterator_array_1[var__iterator_index_3];
///     }
/// }
/// ```
pub fn compile_loop_array_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopArrayIterator,
) -> CompilerResult {
    compiler.enter_variable_scope();

    // Put the array on the stack and assign it to a cache variable
    let array = compiler.define_variable(
        "_iterator_array".to_string(),
        Types::Array(Box::from(Types::Basic(BaseTypes::Integer))),
    );

    // Array
    compiler.add_to_current_function(format!("() {{ auto {} = ", array.transpile()));
    compiler.compile_expression(*lp.array, false);
    compiler.add_to_current_function(";".to_string());

    // Define the identifier variable, with the starting value of the array
    let var = compiler.define_variable(lp.identifier.value, Types::Basic(BaseTypes::Integer));

    let index = compiler.define_variable(
        "_iterator_index".to_string(),
        Types::Basic(BaseTypes::Integer),
    );

    compiler.add_to_current_function(format!("int {} = 0;", index.transpile()));
    compiler.add_to_current_function(format!(
        "auto {} = {}[0];",
        var.transpile(),
        array.transpile()
    ));

    compiler.add_to_current_function(format!(
        "while({} < {}.length) {{ ",
        index.transpile(),
        array.transpile()
    ));

    // Compile body and then increase the index
    let result = compiler.compile_loop_block(lp.body);

    compiler.add_to_current_function(format!("{} += 1;", index.transpile()));
    compiler.add_to_current_function(format!(
        "if({} < {}.length) {{ {} = {}[{}]; }} }} }}();",
        index.transpile(),
        array.transpile(),
        var.transpile(),
        array.transpile(),
        index.transpile()
    ));

    result
}
