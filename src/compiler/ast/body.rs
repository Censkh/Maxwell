use super::statement::StatementNode;

use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct BodyNode {
    pub content: Vec<StatementNode>,
}

impl BodyNode {
    pub fn new(content: Vec<StatementNode>) -> Self {
        return BodyNode {
            content
        };
    }
}

impl Node for BodyNode {
    fn generate(&self) -> String {
        let mut string = String::new();
        for statement in &self.content {
            string += &statement.generate();
        }
        return string;
    }
}