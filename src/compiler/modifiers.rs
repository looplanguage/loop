/// Modifiers of a variable, examples: `const`, `pub`
#[derive(Default, Clone, Debug)]
pub struct Modifiers {
    /// Syntax: `const`
    ///
    /// Field is a true of false
    ///  - `true`  -> constant
    ///  - `false` -> mutable
    pub constant: bool, // If a variable is constant, default is false
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
    ///
    /// ```
    /// let mod: Modifiers = Modifiers::new(true);
    /// ```
    pub fn new(constant: bool) -> Modifiers {
        Modifiers { constant }
    }
}
