use super::Literal;
use super::Keyword;
use super::OperatorKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Semicolon,
    Colon,
    Comma,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,
    Operator(OperatorKind),
    Identifier(String),
    Literal(Literal),
    Keyword(Keyword),
    EndOfFile,
    Whitespace(i8),
    Newline
}

impl<> ToString for Token {
    fn to_string<'a>(&self) -> String {
        use self::Token::*;

        let str = match *self {
            BraceClose => String::from("}"),
            BraceOpen => "{".to_owned(),
            BracketClose => ")".to_owned(),
            BracketOpen => "(".to_owned(),
            Semicolon => ";".to_owned(),
            Colon => ":".to_owned(),
            Comma => ",".to_owned(),
            Operator(ref operator) => operator.to_string(),
            Identifier(ref string) => string.to_string(),
            Keyword(ref keyword) => keyword.to_string(),
            Literal(ref literal) => literal.to_string(),
            EndOfFile => "".to_owned(),
            Whitespace(ref count) => {
                let mut str = "".to_owned();
                for _ in 0..*count {
                    str += " ";
                }
                return str;
            },
            Newline => "\\n".to_owned(),
        };
        return str;
    }
}