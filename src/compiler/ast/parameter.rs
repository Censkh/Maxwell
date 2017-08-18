use super::super::Literal;

#[derive(Debug, PartialEq, Clone)]
pub struct ParameterTrivia {
    pub prefix : String,
    pub suffix : String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub default: Option<Literal>,
    pub trivia: ParameterTrivia,
}