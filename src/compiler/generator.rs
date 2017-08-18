use super::ast::Node;

pub struct Generator {}

impl Generator {
    pub fn generate(&self, node: &Node) -> String {
        return node.generate();
    }
}