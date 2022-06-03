use crate::ast::instructions::Node;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
pub struct While {
    pub condition: Node,
    pub body: Vec<Node>,
}

impl Display for While {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WHILE CONDITION {{{}}} THEN {{{:?}}};",
            self.condition, self.body
        )
    }
}
