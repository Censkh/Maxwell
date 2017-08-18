#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Undefined,
    Null,
    Boolean(bool),
    Binary(u64),
    Number(String),
    String(String, QuoteKind),

}

#[derive(Debug, PartialEq, Clone)]
pub enum QuoteKind {
    SpeechMark,
    Apostrophe
}

impl ToString for QuoteKind {
    fn to_string(&self) -> String {
        return match *self {
            QuoteKind::Apostrophe => "'",
            QuoteKind::SpeechMark => "\"",
        }.to_owned();
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        use self::Literal::*;

        let str = match *self {
            Null => "null".to_owned(),
            Binary(ref binary) => binary.to_string(),
            Undefined => "undefined".to_owned(),
            Boolean(true) => "true".to_owned(),
            Boolean(false) => "false".to_owned(),
            String(ref string, ref quote) => format!("{}{}{}", quote.to_string(), string, quote.to_string()),
            Number(ref string) => string.to_string(),
        };
        return str;
    }
}