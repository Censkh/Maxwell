use super::parser::ParserError;
use super::ConfigError;

#[derive(Debug)]
pub enum CompilerError {
    InvalidConfig(ConfigError),
    ParserError(ParserError),
    EntryNotValid(String),
    NoEntries
}

impl ToString for CompilerError {
    fn to_string(&self) -> String {
        use self::CompilerError::*;
        use self::ConfigError::*;

        match self {
            &InvalidConfig(ref config_err) => match config_err {
                &FileNotFound(ref path) => format!(" File not found at path {}", path),
                &JsonInvalid => " Invalid json contents".to_owned(),
                &FileContentsInvalid => " File contents are invalid".to_owned(),
                &InvalidConfigOption(ref option, ref reason) => format!(" Invalid config option '{}': {}", option, reason),
            },
            &ParserError(ref parser_err) => format!(" {:?}", parser_err),
            &EntryNotValid(ref entry_name) => format!(" Invalid entry file {}", entry_name),
            &NoEntries => format!("{}", " No entries defined")
        }
    }
}