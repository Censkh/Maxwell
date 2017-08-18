use super::super::{Chunk, ChunkLocation};

use std::path::PathBuf;

#[derive(Debug)]
pub enum ResolverError {
    ImportNotFound(String)
}

pub struct Resolver {}

impl Resolver {
    pub fn resolve(&self, chunk: &Chunk, requires: &Vec<String>) -> Result<Vec<ChunkLocation>, ResolverError> {
        let mut new_chunks = Vec::new();
        for import in requires {
            let location = ChunkLocation::relative(&chunk.location, PathBuf::from(import));
            if location.is_ok() {
                new_chunks.push(location.unwrap());
            } else {
                return Err(ResolverError::ImportNotFound(import.clone()));
            }
        }
        return Ok(new_chunks);
    }
}