use super::body::BodyNode;
use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct SyntaxTree {
    pub base_node: Box<BodyNode>,
}

impl SyntaxTree {
    pub fn new(base_node: BodyNode) -> Self {
        return SyntaxTree { base_node: Box::new(base_node) };
    }
}

impl Node for SyntaxTree {
    fn generate(&self) -> String {
        return self.base_node.generate();
    }
}


