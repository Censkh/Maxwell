pub trait Node {
    fn generate(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeTrivia {
    pub prefix: String,
    pub suffix: String,
}

impl NodeTrivia {
    pub fn new() -> Self {
        return NodeTrivia { prefix: String::new(), suffix: String::new() };
    }
}

