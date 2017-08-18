use super::expression::{ExpressionNode, Expression};
use super::declaration::{DeclarationKind, DeclarationNode};
use super::{NodeTrivia, SourceLocation, Node};
use super::super::QuoteKind;

#[derive(Debug, PartialEq, Clone)]
pub struct ImportTrivia {
    pub declaration_prefix: String,
    pub as_prefix: String,
    pub alias_prefix: String,
    pub from_prefix: String,
    pub source_prefix: String,
    pub quote_kind: QuoteKind,
}

impl ImportTrivia {
    pub fn new() -> Self {
        return ImportTrivia {
            declaration_prefix: String::new(),
            as_prefix: String::new(),
            from_prefix: String::new(),
            alias_prefix: String::new(),
            source_prefix: String::new(),
            quote_kind: QuoteKind::Apostrophe,
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportDeclaration {
    All,
    Single(String),
    Multiple(Vec<String>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression {
        expression: ExpressionNode
    },
    Declaration {
        kind: DeclarationKind,
        declarations: Vec<DeclarationNode>
    },
    Return {
        expression: Option<ExpressionNode>
    },
    Import {
        alias: Option<String>,
        declaration: ImportDeclaration,
        source: String,
        trivia: ImportTrivia
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatementTerminator {
    Semicolon,
    Newline,
    Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StatementNode {
    pub statement: Statement,
    pub location: SourceLocation,
    pub trivia: NodeTrivia,
    pub terminator: StatementTerminator,
    pub requires: Vec<String>,
}

impl StatementNode {
    pub fn new(statement: Statement, trivia: NodeTrivia, terminator: StatementTerminator) -> Self {
        return StatementNode {
            statement,
            location: SourceLocation { start: 0, end: 0 },
            trivia,
            terminator,
            requires: Vec::new(),
        };
    }
}

impl Node for StatementNode {
    fn generate(&self) -> String {
        use self::Statement::*;

        let mut string = match self.statement {
            Declaration { ref kind, ref declarations } => {
                let mut string = String::new();
                for i in 0..declarations.len() {
                    string += match i {
                        0 => "",
                        _ => ","
                    };
                    string += &declarations[i].generate();
                }
                format!("{} {}", kind.to_string(), string)
            }
            Expression { ref expression } => expression.generate().to_owned(),
            Return { ref expression } => {
                format!("return{}", match expression {
                    &Some(ref value) => value.generate(),
                    &None => "".to_owned()
                })
            }
            Import { ref alias, ref declaration, ref source, ref trivia } => {
                let declaration_string = match declaration {
                    &ImportDeclaration::All => String::from("*"),
                    &ImportDeclaration::Single(ref name) => name.to_owned(),
                    &ImportDeclaration::Multiple(ref names) => names.join(",")
                };
                let alias_string = match alias {
                    &Some(ref name) => {
                        format!("{}as{}{}", trivia.as_prefix, trivia.alias_prefix, name)
                    }
                    &None => String::new(),
                };
                //TODO: We don't store the literal quote type. Could be "" or ''
                format!("import{}{}{}{}from{}{}{}{}",
                        trivia.declaration_prefix,
                        declaration_string,
                        alias_string,
                        trivia.from_prefix,
                        trivia.source_prefix,
                        trivia.quote_kind.to_string(),
                        source.to_owned(),
                        trivia.quote_kind.to_string())
            }
        };
        return format!("{}{}{}{}", self.trivia.prefix, string, self.trivia.suffix, match self.terminator {
            StatementTerminator::Semicolon => ";",
            StatementTerminator::Newline => "\n",
            _ => ""
        });
    }
}