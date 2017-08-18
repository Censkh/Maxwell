use super::super::{ChunkId};

#[derive(Debug)]
pub struct Dependencies {
    pub requires: Vec<ChunkId>,
    pub used_by: Vec<ChunkId>
}

impl Dependencies {
    pub fn new() -> Self {
        return Dependencies {
            requires: Vec::new(),
            used_by: Vec::new()
        };
    }

}