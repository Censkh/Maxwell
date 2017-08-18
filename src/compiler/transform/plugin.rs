use super::super::{Chunk};

use super::super::ast::statement::StatementNode;
use super::super::ast::expression::ExpressionNode;

use std::result::Result;
use std::error::Error;
use std::hash::{Hash,Hasher};

pub trait Plugin {
    fn handle(&self, pass: &mut PluginPass) -> Result<String, Box<Error>>;

    fn get_name(&self) -> &str;
}

impl Hash for Plugin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_name().hash(state);
    }
}

impl Eq for Plugin {}

impl PartialEq for Plugin {
    fn eq(&self, other: &Plugin) -> bool {
        return self.get_name().eq(other.get_name());
    }
}

pub enum PluginPass<'a> {
    EmitChunk(&'a mut Chunk),
    StatementNodeEmit(&'a mut StatementNode),
    ExpressionNodeEmit(&'a mut ExpressionNode)
}