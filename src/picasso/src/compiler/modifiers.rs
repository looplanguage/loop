/// Modifiers of a variable, examples: `const`, `pub`
#[derive(Default, Clone, Debug)]
pub struct Modifiers {
    /// Syntax: `const`
    ///
    /// Field is a true of false
    ///  - `true`  -> constant
    ///  - `false` -> mutable
    pub constant: bool, // If a variable is constant, default is false
    pub public: bool,
    pub module: String, // Location of the variable
}

impl Modifiers {
    /// Instantiates a new [Modifiers] struct
    ///
    /// # Arguments
    ///
    /// * `constant`: bool
    ///
    /// returns: Modifiers
    ///
    /// # Examples
    #[allow(dead_code)]
    pub fn new(constant: bool, module: String, public: bool) -> Modifiers {
        Modifiers { constant, module, public }
    }
}
