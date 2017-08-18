use super::lexicon;
use super::module::Dependencies;
use super::ast::SyntaxTree;

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::io::prelude::*;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type ChunkId = u64;

#[derive(Debug,Clone)]
pub struct ChunkLocation {
    pub path: PathBuf,
}

impl ChunkLocation {
    pub fn absolute(mut path: PathBuf) -> Result<Self, Error> {
        path = path.with_extension("js");
        if !path.is_absolute() || !path.exists() {
            return Err(Error::from(ErrorKind::NotFound));
        }
        return Ok(ChunkLocation { path });
    }

    pub fn relative(source: &ChunkLocation, mut path: PathBuf) -> Result<Self, Error> {
        let from_dir = source.path.parent().unwrap();
        path = from_dir.join(path).with_extension("js");
        if !path.is_absolute() || !path.exists() {
            return Err(Error::from(ErrorKind::NotFound));
        }
        return Ok(ChunkLocation { path });
    }

    pub fn generate_id(&self) -> ChunkId {
        let mut hasher = DefaultHasher::new();
        self.path.hash(&mut hasher);
        return hasher.finish();
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub name : String,
    pub location: ChunkLocation,
    pub loaded: bool,
    pub index: usize,
    pub source: String,
    pub syntax_tree: Option<SyntaxTree>,
    pub dependencies: Dependencies,
}

impl<'a> Chunk {
    pub fn new(name : String, location: ChunkLocation) -> Self {
        return Chunk {
            name,
            loaded:false,
            location,
            source: String::new(),
            index: 0,
            syntax_tree: None,
            dependencies: Dependencies::new()
        };
    }

    pub fn get_id(&self) -> ChunkId {
        return self.location.generate_id();
    }

    pub fn update_syntax_tree(&mut self, tree : SyntaxTree) {
        self.syntax_tree = Some(tree);
    }

    pub fn load_source(&mut self) -> Result<&str, Error> {
        let file = File::open(&self.location.path);
        if file.is_err() {
            return Err(file.err().unwrap());
        }

        let mut src = String::new();
        let result = file.unwrap().read_to_string(&mut src);
        match result {
            Ok(_) => {}
            Err(err) => return Err(err)
        };

        //TODO: Do we need to copy the memory?
        self.source = src;
        self.loaded = true;
        return Ok(&self.source);
    }

    pub fn peek_char(&self) -> char {
        return self.char_at(self.index);
    }

    pub fn len(&self) -> usize {
        return self.source.len();
    }

    pub fn is_eof(&self) -> bool {
        return self.index >= self.len();
    }

    pub fn consume_label(&mut self) -> &str {
        let start = self.index;
        //We bump the first character because we ASSUME it is an ident char if this method is being called.
        self.bump_char();

        loop {
            if self.is_eof() { break; }
            let char = self.peek_char();
            if lexicon::is_ident(char) {
                self.bump_char();
            } else {
                break;
            }
        }
        unsafe {
            return self.source.slice_unchecked(start, self.index);
        }
    }

    pub fn slice(&'a self, start: usize, end: usize) -> &'a str {
        unsafe {
            return self.source.slice_unchecked(start, end);
        }
    }

    pub fn bump_char(&mut self) -> usize {
        self.index += 1;
        return self.index;
    }

    pub fn char_at(&self, index: usize) -> char {
        return self.source.chars().nth(index).unwrap();
    }
}