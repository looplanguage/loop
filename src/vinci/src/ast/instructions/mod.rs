use crate::ast::instructions::conditional::Conditional;
use crate::ast::instructions::function::{Call, Function};
use crate::ast::instructions::memory::{
    CompoundType, Copy, Index, Load, LoadLib, Push, Slice, Store,
};
use crate::ast::instructions::suffix::Suffix;
use crate::ast::instructions::while_loop::While;
use crate::types::ValueType;
use std::fmt::{Debug, Display, Formatter};

pub mod conditional;
pub mod function;
pub mod memory;
pub mod suffix;
pub mod while_loop;

#[derive(PartialEq, Clone)]
pub enum Node {
    /// A value without an assignment
    /// ```txt
    /// .CONSTANT INT 30;
    /// .CONSTANT CHAR[] "Hello, world!";
    /// ```
    CONSTANT(ValueType),
    /// Loading a variable from a specific location
    /// ```txt
    /// .LOAD VARIABLE 0;
    /// .LOAD PARAMETERS 2;
    /// ```
    LOAD(Load),
    /// Storing a value in memory by location
    /// ```txt
    /// .STORE VARIABLE 0 { .CONSTANT INT 10; }
    /// .STORE VARIABLE 1 { .LOAD PARAMETER 1; }
    /// ```
    STORE(Store),
    /// Binary operators like `add`, `subtract`, etc.
    /// ```txt
    /// .ADD { .CONSTANT INT 10; } { .LOAD VARIABLE 10; }
    /// ```
    SUFFIX(Box<Suffix>),
    /// A if-expression
    /// ```txt
    /// .IF
    /// CONDITION { .CONSTANT BOOL true; } THEN {
    ///    .STORE VARIABLE 0 { .CONSTANT INT 10; }
    /// } ELSE {
    ///    .STORE VARIABLE 1 { .CONSTANT INT 11; }
    /// };
    /// ```
    CONDITIONAL(Box<Conditional>),
    /// Defining a function, executing the body when the conditino evaluates to `true`
    /// ```txt
    /// .FUNCTION "name" INT PARAMETERS { INT; INT; } THEN { .STORE VARIABELE 1 { .LOAD VARIABLE 2; } };
    /// ```
    FUNCTION(Box<Function>),
    /// Calling a previous defined function
    /// ```txt
    /// .CALL local::double { .LOAD VARIABLE 1; };
    /// ```
    CALL(Box<Call>),
    /// Returning from a function
    /// ```txt
    /// .RETURN { .CONSTANT INT 400; }
    /// ```
    RETURN(Box<Node>),
    /// A loop, executes the block as long as the expression is `true`
    /// ```txt
    /// .WHILE CONDITION { .CONSTANT BOOL true; } THEN {
    ///     .CONSTANT INT 10;
    /// };
    /// ```
    WHILE(Box<While>),
    /// Instruction to index a array or string. Starting from 0
    /// ```txt
    /// .INDEX
    /// {
    ///     .CONSTANT INT[] [10,20,30];
    /// };
    /// {
    ///     .CONSTANT INT 0;
    /// };
    /// ```
    INDEX(Index),
    /// Instruction to slice an array or string. First is inclusive, end is exclusive. Starting from 0
    /// ```txt
    /// .SLICE
    /// { .CONSTANT INT 0; }
    /// { .CONSTANT INT 1; }
    /// { .CONSTANT INT[] [10,20]; }
    /// ```
    SLICE(Slice),
    /// Pushing a value to array or string
    /// ```txt
    /// .PUSH { .CONSTANT INT[] [10,20,30]; } { .CONSTANT INT[] [40, 50]; };
    /// // Resulting in [10, 20, 30, 40, 50]
    /// ```
    PUSH(Push),
    // TODO
    COPY(Copy),
    /// Loading of dynamic library. Expecting library to provide a header file in the form of either a `function_signatures` or a header file.
    /// ```txt
    /// .LOADLIB { .CONSTANT CHAR[] "../../dasd" } { .CONSTANT CHAR[] "namespace" };
    /// ```
    LOADLIB(LoadLib),
    /// Assigning a value to a variable
    /// ```txt
    /// .INDEX { .CONSTANT INT[] [10,20,30]; }; { .CONSTANT INT 0; };
    /// ```
    ASSIGN(Box<Node>, Box<Node>),
    /// Removing an element from an array or string
    /// ```txt
    /// .POP { .CONSTANT INT[] [10,20]; } { .CONSTANT INT 1; }
    /// // Resulting in [ 10 ]
    /// ```
    POP(Box<Node>, Box<Node>),
    // TODO
    LENGTH(Box<Node>),
    /// Provide two conditions, with one of them being two to evaluate to `true`
    AND(Box<Node>, Box<Node>),
    /// Provide two conditions, with both of them being two to evaluate to `true`
    OR(Box<Node>, Box<Node>),
    /// Defining a new type, consisting of different types
    /// ```txt
    /// .COMPOUND { INT; STRING; }
    /// ```
    COMPOUND(CompoundType),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::LOAD(load) => write!(f, "{}", load),
            Node::SUFFIX(suffix) => write!(f, "{}", suffix),
            Node::STORE(store) => write!(f, "{}", store),
            Node::CONSTANT(value_type) => write!(f, "{}", value_type),
            Node::CONDITIONAL(conditional) => write!(f, "{}", conditional),
            Node::FUNCTION(func) => write!(f, "{}", func),
            Node::CALL(call) => write!(f, "{}", call),
            Node::WHILE(wh) => write!(f, "{}", wh),
            Node::PUSH(wh) => write!(f, "{}", wh),
            Node::SLICE(wh) => write!(f, "{}", wh),
            Node::INDEX(wh) => write!(f, "{}", wh),
            Node::LOADLIB(loadlib) => write!(f, "{}", loadlib),
            Node::COPY(copy) => write!(f, "{}", copy),
            Node::RETURN(ret) => write!(f, "{}", ret),
            Node::POP(a, b) => write!(f, "{}, {}", a, b),
            Node::ASSIGN(a, b) => write!(f, "{}, {}", a, b),
            Node::LENGTH(a) => write!(f, "{}", a),
            Node::AND(a, b) => write!(f, "{}, {}", a, b),
            Node::OR(a, b) => write!(f, "{}, {}", a, b),
            Node::COMPOUND(cmp) => write!(f, "{:?}", cmp),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
