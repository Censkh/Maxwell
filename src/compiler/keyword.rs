use super::ast::declaration::DeclarationKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Function,
    Default,
    While,
    Finally,
    With,
    Switch,
    Yield,
    Break,
    Do,
    In,
    Of,
    Class,
    Extends,
    Return,
    Import,
    Export,
    Try,
    As,
    Catch,
    If,
    From,
    Async,
    Await,
    Declaration(DeclarationKind),
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        use self::Keyword::*;

        let string = match *self {
            From => "from".to_owned(),
            Function => "function".to_owned(),
            Break => "break".to_owned(),
            Catch => "catch".to_owned(),
            If => "if".to_owned(),
            Class => "class".to_owned(),
            Default => "default".to_owned(),
            Yield => "yield".to_owned(),
            Async => "async".to_owned(),
            While => "while".to_owned(),
            As => String::from("as"),
            Finally => "finally".to_owned(),
            Switch => "switch".to_owned(),
            Return => "return".to_owned(),
            Extends => "extends".to_owned(),
            Try => "try".to_owned(),
            Await => "await".to_owned(),
            Do => "do".to_owned(),
            Export => "export".to_owned(),
            Import => "import".to_owned(),
            With => "with".to_owned(),
            In => String::from("in"),
            Of => String::from("of"),

            Declaration(ref declaration) => declaration.to_string(),
        };
        return string;
    }
}
