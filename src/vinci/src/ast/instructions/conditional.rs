use crate::ast::instructions::Node;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
pub struct Conditional {
    pub condition: Node,
    pub body: Vec<Node>,
    pub alternative: Vec<Node>,
}

impl Display for Conditional {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".IF .CONDITION {{{}}} .THEN {{{:?}}} .ELSE {{{:?}}}",
            self.condition, self.body, self.alternative
        )
    }
}
