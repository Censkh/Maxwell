#[derive(Debug, PartialEq, Clone)]
pub struct SourceLocation {
    pub start: usize,
    pub end: usize,
}

impl SourceLocation {
    pub fn new(start: usize, end: usize) -> Self {
        return SourceLocation { start, end };
    }

    pub fn default() -> Self {
        return SourceLocation::new(0, 0);
    }
}