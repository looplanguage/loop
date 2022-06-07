//! Contains compilation functions for all nodes
/// Transpiling an array to D code
pub mod expression_array;
/// Transpiling a bool to D code
pub mod expression_bool;
/// Transpiling function calls to D code
pub mod expression_call;
/// Transpiling if-expressions to D code
pub mod expression_conditional;
/// Transpiling floats to D code
pub mod expression_float;
/// Transpiling functions to D code
pub mod expression_function;
/// Transpiling hashmaps to D code
pub mod expression_hashmap;
/// Transpiling identifiers to D code
pub mod expression_identifier;
/// Transpiling indexing (arrays, hashmaps, etc) to D code
pub mod expression_index;
/// Transpiling integers to D code
pub mod expression_integer;
/// Transpiling loops to D code
pub mod expression_loop;
/// Transpiling nulls to D code
pub mod expression_null;
/// Transpiling strings to D code
pub mod expression_string;
/// Transpiling suffix expressions to D code
pub mod expression_suffix;
/// Transpiling breaks to D code
pub mod statement_break;
pub mod statement_class;
/// Transpiling constant declarations to D code
pub mod statement_constant_declaration;
/// Transpiling exports to D code
pub mod statement_export;
pub mod statement_extend;
/// Transpiling imports to D code
pub mod statement_import;
/// Transpiling returns to D code
pub mod statement_return;
/// Transpiling variable assigning to D code
pub mod statement_variable_assign;
/// Transpiling variable declaration to D code
pub mod statement_variable_declaration;
