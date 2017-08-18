use super::super::ast::SourceLocation;

#[derive(Debug)]
pub enum ParserErrorKind {
    Syntax,
}

#[derive(Debug)]
pub struct ParserError {
    kind: ParserErrorKind,
    description: String,
    location: SourceLocation,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, description: String, location: SourceLocation) -> Self {
        return ParserError { kind, description, location };
    }
}