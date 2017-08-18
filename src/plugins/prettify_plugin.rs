use compiler::transform::PluginPass;
use compiler::transform::Plugin;

use std::result::Result;
use std::error::Error;

use compiler::ast::statement::StatementTerminator;

use compiler::ast::expression::Expression;

pub struct PrettifyPlugin {}

impl Plugin for PrettifyPlugin {
    fn handle(&self, pass: &mut PluginPass) -> Result<String, Box<Error>> {
        use self::PluginPass::*;
        use self::Expression::*;

        match pass {
            &mut StatementNodeEmit(ref mut statement_node) => {
                statement_node.trivia.prefix = String::from("\n");
                if statement_node.terminator == StatementTerminator::Newline {
                    statement_node.terminator = StatementTerminator::Semicolon;
                }
                Ok(String::from("hello world"))
            }
            &mut ExpressionNodeEmit(ref mut expression_node) => {
                match expression_node.expression {
                    Function { ref mut trivia, .. } => {
                        //TODO: trivia.cleanup()
                        trivia.identifier_gap = String::from(" ");
                        trivia.parameters_gap = String::from("");
                        trivia.body_gap = String::from(" ");
                        trivia.body_suffix = String::from("\n");
                        trivia.parameters_padding = String::from("");

                        Ok(String::from(""))
                    }
                    _ => Ok(String::from(""))
                }
            }
            _ => Ok("".to_owned())
        }
    }

    fn get_name(&self) -> &str {
        return "internal.prettify";
    }
}