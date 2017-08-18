use super::expression::ExpressionNode;
use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationTrivia {
    pub prefix: String,
    pub assign_prefix: String
}

impl DeclarationTrivia {
    pub fn new() -> Self {
        return DeclarationTrivia { prefix: String::new(), assign_prefix: String::new() };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationNode {
    pub name: String,
    pub expression: Option<ExpressionNode>,
    pub trivia: DeclarationTrivia,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationKind {
    Let,
    Const,
    Var,
}

impl DeclarationNode {
    pub fn new(name: String, expression: Option<ExpressionNode>, trivia: DeclarationTrivia) -> Self {
        return DeclarationNode { name, expression, trivia };
    }
}

impl Node for DeclarationNode {
    fn generate(&self) -> String {
        let mut expression = String::new();
        match self.expression {
            Some(ref expression_node) => {
                expression += " = ";
                expression += &expression_node.generate();
            }
            None => {}
        }
        return format!("{}{}", self.name, expression);
    }
}

impl ToString for DeclarationKind {
    fn to_string(&self) -> String {
        use self::DeclarationKind::*;

        return match self {
            &Const => "const",
            &Let => "let",
            &Var => "var",
        }.to_owned();
    }
}