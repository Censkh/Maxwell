use super::lexicon;
use super::ast::declaration::DeclarationKind;
use super::{Chunk,Token,Keyword,QuoteKind,Literal,OperatorKind};

use std::result::Result;
use std::error::Error;
use std::fmt;

pub struct Tokenizer {
}

#[derive(Debug)]
pub struct TokenizerError {
    description: String
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenizer Error: {}", self.description())
    }
}

impl TokenizerError {
    pub fn new(description: &str) -> Self {
        return TokenizerError { description: description.to_owned() };
    }
}

impl Error for TokenizerError {
    fn description(&self) -> &str {
        return &self.description;
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer {};
    }

    pub fn peek_token(&mut self, chunk: &mut Chunk) -> Result<Token, TokenizerError> {
        let start_index = chunk.index;
        let result = self.pop_token(chunk);

        chunk.index = start_index;
        return result;
    }

    pub fn pop_token(&mut self, chunk: &mut Chunk) -> Result<Token, TokenizerError> {
        if chunk.is_eof() {
            return Ok(Token::EndOfFile);
        }

        let char = chunk.peek_char();

        if char == '\n' {
            chunk.bump_char();
            return Ok(Token::Newline);
        }

        if char == '\r' {
            chunk.bump_char();
            if !chunk.is_eof() && chunk.peek_char() == '\n' {
                chunk.bump_char();
            }
            return Ok(Token::Newline);
        }

        if char.is_whitespace() {
            let mut count = 1;
            chunk.bump_char();
            loop {
                if chunk.is_eof() {
                    break;
                }
                if chunk.peek_char().is_whitespace() {
                    count += 1;
                    chunk.bump_char();
                } else {
                    break;
                }
            }
            return Ok(Token::Whitespace(count));
        }

        match char {
            '=' => {
                chunk.bump_char();
                let operator = match chunk.peek_char() {
                    '=' => {
                        chunk.bump_char();
                        match chunk.peek_char() {
                            '=' => {
                                chunk.bump_char();
                                OperatorKind::StrictEquality
                            }
                            _ => OperatorKind::Equality
                        }
                    }
                    '>' => {
                        chunk.bump_char();
                        return Ok(Token::FatArrow);
                    }
                    _ => OperatorKind::Assign
                };
                return Ok(Token::Operator(operator));
            }
            '(' => {
                chunk.bump_char();
                return Ok(Token::BracketOpen);
            }
            ')' => {
                chunk.bump_char();
                return Ok(Token::BracketClose);
            }
            '{' => {
                chunk.bump_char();
                return Ok(Token::BraceOpen);
            }
            '}' => {
                chunk.bump_char();
                return Ok(Token::BraceClose);
            }
            ';' => {
                chunk.bump_char();
                return Ok(Token::Semicolon);
            }
            ':' => {
                chunk.bump_char();
                return Ok(Token::Colon);
            }
            '.' => {
                let start = chunk.index;
                chunk.bump_char();
                let token = match chunk.peek_char() {
                    '0' ... '9' => Token::Literal(Literal::Number(self.read_number(chunk, start).to_owned())),
                    '.' => {
                        unimplemented!();
                    }
                    _ => Token::Operator(OperatorKind::Accessor)
                };
                return Ok(token);
            }
            ',' => {
                chunk.bump_char();
                return Ok(Token::Comma);
            }
            '0' ... '9' => {
                let start = chunk.index;
                return Ok(Token::Literal(Literal::Number(self.read_number(chunk, start).to_owned())));
            }
            '\'' => Ok(self.read_quote_until(chunk, '\'')),
            '"' => Ok(self.read_quote_until(chunk, '"')),
            _ => {
                if lexicon::is_ident(char) {
                    let label = chunk.consume_label();
                    return match label {
                        "if" => Ok(Token::Keyword(Keyword::If)),
                        "function" => Ok(Token::Keyword(Keyword::Function)),
                        "return" => Ok(Token::Keyword(Keyword::Return)),
                        "while" => Ok(Token::Keyword(Keyword::While)),
                        "switch" => Ok(Token::Keyword(Keyword::Switch)),

                        "finally" => Ok(Token::Keyword(Keyword::Function)),
                        "with" => Ok(Token::Keyword(Keyword::Return)),
                        "in" => Ok(Token::Keyword(Keyword::In)),
                        "break" => Ok(Token::Keyword(Keyword::Break)),
                        "do" => Ok(Token::Keyword(Keyword::Do)),

                        "class" => Ok(Token::Keyword(Keyword::Class)),
                        "extends" => Ok(Token::Keyword(Keyword::Extends)),

                        "async" => Ok(Token::Keyword(Keyword::Async)),
                        "await" => Ok(Token::Keyword(Keyword::Await)),

                        "import" => Ok(Token::Keyword(Keyword::Import)),
                        "export" => Ok(Token::Keyword(Keyword::Export)),

                        "from" => Ok(Token::Keyword(Keyword::From)),
                        "try" => Ok(Token::Keyword(Keyword::Try)),
                        "catch" => Ok(Token::Keyword(Keyword::Catch)),


                        "of" => Ok(Token::Keyword(Keyword::Of)),
                        "yield" => Ok(Token::Keyword(Keyword::Yield)),

                        "let" => Ok(Token::Keyword(Keyword::Declaration(DeclarationKind::Let))),
                        "var" => Ok(Token::Keyword(Keyword::Declaration(DeclarationKind::Var))),
                        "const" => Ok(Token::Keyword(Keyword::Declaration(DeclarationKind::Const))),
                        "null" => Ok(Token::Literal(Literal::Null)),
                        "undefined" => Ok(Token::Literal(Literal::Undefined)),
                        "true" => Ok(Token::Literal(Literal::Boolean(true))),
                        "false" => Ok(Token::Literal(Literal::Boolean(false))),
                        value => Ok(Token::Identifier(value.to_owned()))
                    };
                } else {
                    return Err(TokenizerError::new("Invalid token."));
                }
            }
        }
    }

    fn read_quote_until<'a>(&self, chunk: &'a mut Chunk, quote_char: char) -> Token {
        let start = chunk.index + 1;

        chunk.bump_char();

        loop {
            match chunk.peek_char() {
                '\\' => {
                    chunk.bump_char();
                    chunk.bump_char();
                }
                char => {
                    if char == quote_char { break; }
                    chunk.bump_char();
                }
            }
        }

        let end = chunk.index;
        let value = chunk.slice(start, end).to_owned();
        chunk.bump_char();

        let quote = match quote_char {
            '"' => QuoteKind::SpeechMark,
            '\'' => QuoteKind::Apostrophe,
            _ => panic!("Invalid char")
        };

        return Token::Literal(Literal::String(value,quote));
    }

    fn read_number<'a>(&self, chunk: &'a mut Chunk, start: usize) -> &'a str {
        while !chunk.is_eof() {
            match chunk.peek_char() {
                '0' ... '9' => {
                    chunk.bump_char();
                }
                '.' => {
                    chunk.bump_char();
                }
                _ => break
            }
        }

        return chunk.slice(start, chunk.index);
    }

    pub fn peek_ignore_whitespace<'a>(&mut self, chunk: &'a mut Chunk) -> (Token, String) {
        use self::Token::*;

        let start_index = chunk.index;

        let mut string = String::new();
        let mut token: Token;

        loop {
            token = self.peek_token(chunk).unwrap();
            match token {
                Whitespace(count) => {
                    for _ in 0..count {
                        string += " ";
                    }
                    self.pop_token(chunk);
                }
                _ => {
                    break;
                }
            }
        }
        chunk.index = start_index;

        return (token, string);
    }

    pub fn pop_ignore_whitespace<'a>(&mut self,chunk: &'a mut Chunk) {
        use self::Token::*;

        loop {
            match self.peek_token(chunk).unwrap() {
                Whitespace(count) => self.pop_token(chunk),
                _ => break,
            };
        }

        self.pop_token(chunk);
    }

    pub fn peek_ignore_padding<'a>(&mut self, chunk: &'a mut Chunk) -> (Token, String) {
        use self::Token::*;

        let start_index = chunk.index;

        let mut string = String::new();
        let mut token: Token;

        loop {
            token = self.peek_token(chunk).unwrap();
            match token {
                Whitespace(count) => {
                    for _ in 0..count {
                        string += " ";
                    }
                    self.pop_token(chunk);
                }
                Newline => {
                    string += "\n";
                    self.pop_token(chunk);
                }
                _ => {
                    break;
                }
            }
        }

        chunk.index = start_index;

        return (token, string);
    }

    pub fn pop_ignore_padding<'a>(&mut self, chunk: &'a mut Chunk) {
        use self::Token::*;

        loop {
            match self.peek_token(chunk).unwrap() {
                Whitespace(count) => self.pop_token(chunk),
                Newline => self.pop_token(chunk),
                _ => break,
            };
        }

        self.pop_token(chunk);
    }

}