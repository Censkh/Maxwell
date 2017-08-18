use super::super::Literal;
use super::Parameter;

use super::body::BodyNode;

use super::SourceLocation;
use super::NodeTrivia;
use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionTrivia {
    pub identifier_gap: String,
    pub parameters_gap: String,
    pub body_gap: String,
    pub parameters_padding: String,
    pub body_suffix: String,
}

impl FunctionTrivia {
    pub fn new() -> Self {
        return FunctionTrivia {
            body_suffix: String::new(),
            identifier_gap: String::new(),
            parameters_gap: String::new(),
            body_gap: String::new(),
            parameters_padding: String::new()
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Void,
    This,
    Function {
        name: String,
        parameters: Vec<Parameter>,
        body: BodyNode,
        trivia: FunctionTrivia
    },
    Call {
        identifier: String,
        parameters: Vec<ExpressionNode>
    },
    Identifier(String),
    Literal(Literal)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNode {
    pub expression: Expression,
    location: SourceLocation,
    pub trivia: NodeTrivia,
}

impl ExpressionNode {
    pub fn new(expression: Expression, trivia: NodeTrivia) -> Self {
        return ExpressionNode {
            expression,
            location: SourceLocation { start: 0, end: 0 },
            trivia
        };
    }
}

impl Node for ExpressionNode {
    fn generate(&self) -> String {
        use self::Expression::*;

        let string = match self.expression {
            //TODO: Cleanup
            Function { ref name, ref parameters, ref body, ref trivia } => {
                let mut params = String::new();
                for i in 0..parameters.len() {
                    params += &format!("{}{}", match i {
                        0 => "",
                        _ => ","
                    }, parameters[i].name);
                }
                let mut string = format!("function{}{}{}({}{}){}{{", trivia.identifier_gap, name, trivia.parameters_gap, trivia.parameters_padding, &params, trivia.body_gap);
                string += &body.generate();
                string += &trivia.body_suffix;
                string += "}";
                string
            }
            Call { ref identifier, ref parameters } => {
                let mut params = String::new();
                for i in 0..parameters.len() {
                    params += &format!("{}{}", match i {
                        0 => "",
                        _ => ","
                    }, parameters[i].generate());
                }
                format!("{}({})", identifier, params)
            }
            This => "this".to_owned(),
            Identifier(ref string) => string.to_owned(),
            Literal(ref literal) => literal.to_string().to_owned(),
            _ => "ERROR".to_owned()
        };
        return format!("{}{}{}", self.trivia.prefix, string, self.trivia.suffix);
    }
}