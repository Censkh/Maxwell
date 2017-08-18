use super::super::{Chunk};
use super::super::ast::{SyntaxTree};
use super::super::transform::PluginManager;
use super::ParserError;

use std::time::Duration;

pub struct ParserOptions<'a> {
    pub chunk: &'a mut Chunk,
    pub plugin_manager: &'a PluginManager,
}

impl<'a> ParserOptions<'a> {
    pub fn new(chunk: &'a mut Chunk, plugin_manager: &'a PluginManager) -> Self {
        return ParserOptions { chunk, plugin_manager };
    }
}

pub struct ParserResult {
    pub syntax_tree: SyntaxTree,
    pub requires: Vec<String>,
    pub duration: Duration,
}

impl ParserResult {

    pub fn new(syntax_tree: SyntaxTree, requires: Vec<String>, duration: Duration) -> Self {
        return ParserResult { syntax_tree,requires, duration };
    }
}

pub trait Parser {

    fn parse(&mut self, options: ParserOptions) -> Result<ParserResult, ParserError>;
}