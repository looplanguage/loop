use crate::ast::instructions::Node;
use std::fmt::{Display, Formatter};

pub mod instructions;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub nodes: Vec<Node>,
}

impl AST {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn new() -> AST {
        AST { nodes: Vec::new() }
    }

    #[cfg(test)]
    pub fn new_mock(nodes: Vec<Node>) -> AST {
        AST { nodes }
    }
}

impl Default for AST {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            write!(f, "{};", node)?;
        }

        Ok(())
    }
}
