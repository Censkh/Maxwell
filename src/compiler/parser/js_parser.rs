use super::{Parser, ParserError, ParserErrorKind, ParserResult, ParserOptions};

use super::super::{Token, Keyword, OperatorKind, Literal, Tokenizer};
use super::super::transform::PluginPass;
use super::super::ast::declaration::{DeclarationKind, DeclarationNode, DeclarationTrivia};
use super::super::ast::expression::{FunctionTrivia, ExpressionNode, Expression};
use super::super::ast::statement::{StatementNode, Statement, StatementTerminator, ImportDeclaration, ImportTrivia};
use super::super::ast::{NodeTrivia, Parameter, ParameterTrivia, SyntaxTree, SourceLocation};

use super::super::ast::body::BodyNode;

use std::time::Instant;

pub struct JsParser {
    tokenizer: Tokenizer,
    requires: Vec<String>,
}

impl Parser for JsParser {
    fn parse(&mut self, mut options: ParserOptions) -> Result<ParserResult, ParserError> {
        use self::Token::*;

        let start = Instant::now();

        let mut contents = Vec::new();

        loop {
            let token = self.tokenizer.peek_token(options.chunk).unwrap();
            if token == EndOfFile {
                break;
            }
            match self.parse_statement(&mut options) {
                Ok(statement_node) => contents.push(statement_node),
                Err(err) => return Err(err)
            }
        }
        let tree = SyntaxTree::new(BodyNode::new(contents));
        let requires = self.requires.clone();
        self.requires.clear();

        return Ok(ParserResult::new(tree, requires, start.elapsed()));
    }
}


impl JsParser {
    pub fn new() -> Self {
        return JsParser {
            tokenizer: Tokenizer::new(),
            requires: Vec::new(),
        };
    }

    fn parse_function(&mut self, options: &mut ParserOptions) -> Result<(Expression, Option<StatementTerminator>), ParserError> {
        use self::Token::*;

        let mut trivia = FunctionTrivia::default();

        let (identifier_token, identifier_gap) = self.tokenizer.peek_ignore_padding(options.chunk);
        self.tokenizer.pop_ignore_padding(options.chunk);
        let identifier = match identifier_token {
            Identifier(ref name) => name.to_owned(),
            _ => return Err(ParserError::new(ParserErrorKind::Syntax, String::from("Function has no identifier."), SourceLocation::default()))
        };
        trivia.identifier_gap = identifier_gap;


        let (bracket_token, bracket_gap) = self.tokenizer.peek_ignore_padding(options.chunk);
        self.tokenizer.pop_ignore_padding(options.chunk);
        match bracket_token {
            BracketOpen => {}
            _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Function params must begin with bracket."), SourceLocation::default()))
        }
        trivia.parameters_gap = bracket_gap;

        let mut parameters = Vec::new();
        loop {
            let (parameter_token, parameter_gap) = self.tokenizer.peek_ignore_padding(options.chunk);
            self.tokenizer.pop_ignore_padding(options.chunk);
            match parameter_token {
                BracketClose => {
                    if parameters.len() == 0 {
                        trivia.parameters_padding += &parameter_gap;
                        break;
                    }
                    return Err(ParserError::new(ParserErrorKind::Syntax, format!("Unexpected end of params."), SourceLocation::default()));
                }
                Identifier(ref ident) => {
                    let (next_token, suffix) = self.tokenizer.peek_ignore_padding(options.chunk);
                    self.tokenizer.pop_ignore_padding(options.chunk);

                    parameters.push(Parameter {
                        name: ident.to_owned(),
                        default: None,
                        trivia: ParameterTrivia { prefix: parameter_gap, suffix }
                    });

                    match next_token {
                        Comma => continue,
                        BracketClose => break,
                        _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Function params contains invalid token."), SourceLocation::default())),
                    }
                }
                _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Invalid function."), SourceLocation::default())),
            }
        }

        let (body_token, body_gap) = self.tokenizer.peek_ignore_padding(options.chunk);

        if body_token != BraceOpen {
            return Err(ParserError::new(ParserErrorKind::Syntax, format!("Function body must begin with brace."), SourceLocation::default()));
        }
        self.tokenizer.pop_ignore_padding(options.chunk);
        trivia.body_gap = body_gap;

        let mut body = Vec::new();
        loop {
            let (token, prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
            match token {
                BraceClose => {
                    trivia.body_suffix = prefix;
                    self.tokenizer.pop_ignore_padding(options.chunk);
                    break;
                }
                _ => match self.parse_statement(options) {
                    Ok(mut statement_node) => {
                        statement_node.trivia.prefix = prefix;
                        self.apply_plugin(options, PluginPass::StatementNodeEmit(&mut statement_node));
                        body.push(statement_node)
                    }
                    Err(err) => return Err(err)
                }
            }
        }

        return Ok((Expression::Function {
            name: identifier,
            parameters,
            body: BodyNode::new(body),
            trivia
        }, Some(StatementTerminator::Block)));
    }

    fn apply_plugin(&self, options: &mut ParserOptions, pass: PluginPass) {
        options.plugin_manager.apply_plugin(pass);
    }

    fn parse_bracket_expression(&mut self, options: &mut ParserOptions) -> Result<Expression, ParserError> {
        use self::Token::*;

        let mut expressions = Vec::new();

        loop {
            let (token, prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
            match token {
                BracketClose => {
                    self.tokenizer.pop_ignore_padding(options.chunk);
                    break;
                }
                Comma => {
                    self.tokenizer.pop_ignore_padding(options.chunk);
                }
                _ => {
                    match self.parse_expression(options) {
                        Ok((child_expression, _)) => {
                            expressions.push(child_expression);
                        }
                        Err(err) => return Err(err)
                    }
                }
            }
        }
        let (possible_fat_arrow, arrow_prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
        if possible_fat_arrow == FatArrow {
            return Err(ParserError::new(ParserErrorKind::Syntax, String::from("ASDasd"), SourceLocation::default()));
        } else {
            let expression = expressions.remove(0);
            return Ok(Expression::Bracketed { expression: Box::new(expression) });
        }
    }


    fn parse_expression(&mut self, options: &mut ParserOptions) -> Result<(ExpressionNode, Option<StatementTerminator>), ParserError> {
        let mut trivia = NodeTrivia::new();
        let mut expression_option: Option<Expression> = None;
        let mut terminator_option: Option<StatementTerminator> = None;

        let (token, prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
        trivia.prefix = prefix;
        match token {
            Token::BracketOpen => {
                self.tokenizer.pop_ignore_padding(options.chunk);
                match self.parse_bracket_expression(options) {
                    Ok(expression) => {
                        expression_option = Some(expression);
                    }
                    Err(err) => return Err(err)
                }
            }
            Token::Literal(ref literal) => {
                expression_option = Some(Expression::Literal(literal.clone()));
                self.tokenizer.pop_ignore_padding(options.chunk);
            }
            Token::Keyword(keyword) => {
                match keyword {
                    Keyword::Function => {
                        self.tokenizer.pop_ignore_padding(options.chunk);
                        match self.parse_function(options) {
                            Ok((expression, terminator)) => {
                                expression_option = Some(expression);
                                terminator_option = terminator;
                            }
                            Err(err) => return Err(err)
                        }
                    }
                    _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Keyword '{:?}' not supported.", keyword), SourceLocation::default()))
                }
            }
            Token::Identifier(ref name) => {
                self.tokenizer.pop_ignore_padding(options.chunk);
                let (next_token, _) = self.tokenizer.peek_ignore_padding(options.chunk);
                match next_token {
                    Token::BracketOpen => {
                        self.tokenizer.pop_ignore_padding(options.chunk);
                        match self.parse_call(options, name.to_owned()) {
                            Ok((expression, terminator)) => {
                                expression_option = Some(expression);
                                terminator_option = terminator;
                            }
                            Err(err) => return Err(err)
                        }
                    }
                    _ => {
                        expression_option = Some(Expression::Identifier(name.to_owned()));
                    }
                }
            }
            _ => {}
        }

        if expression_option == None {
            return Err(ParserError::new(ParserErrorKind::Syntax, format!("Expression could not be parsed. Current token: {:?} at index {}", self.tokenizer.peek_token(options.chunk), options.chunk.index), SourceLocation::default()));
        }

        let mut node = ExpressionNode::new(expression_option.unwrap(), trivia);
        self.apply_plugin(options, PluginPass::ExpressionNodeEmit(&mut node));
        return Ok((node, terminator_option));
    }

    fn parse_call(&mut self, options: &mut ParserOptions, identifier: String) -> Result<(Expression, Option<StatementTerminator>), ParserError> {
        let mut parameters = Vec::new();
        loop {
            let (token, _) = self.tokenizer.peek_ignore_padding(options.chunk);
            match token {
                Token::Comma => continue,
                Token::BracketClose => break,
                _ => match self.parse_expression(options) {
                    Ok((expression_node, _)) => {
                        parameters.push(expression_node)
                    }
                    Err(err) => return Err(err)
                }
            }
        }

        let token = self.tokenizer.peek_token(options.chunk).unwrap();
        match token {
            Token::BracketClose => self.tokenizer.pop_token(options.chunk),
            _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Call not closed with bracket"), SourceLocation::default()))
        };

        return Ok((Expression::Call { identifier, parameters }, None));
    }

    fn parse_declaration(&mut self, options: &mut ParserOptions, kind: &DeclarationKind) -> Result<(Statement, Option<StatementTerminator>), ParserError> {
        use self::Token::*;

        let mut declarations = Vec::new();

        loop {
            let (identifier_token, identifier_prefix) = self.tokenizer.peek_ignore_whitespace(options.chunk);

            match identifier_token {
                Identifier(ref name) => {
                    self.tokenizer.pop_ignore_whitespace(options.chunk);
                    let mut trivia = DeclarationTrivia::new();
                    let expression;

                    let (next_token, next_prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
                    match next_token {
                        Operator(OperatorKind::Assign) => {
                            self.tokenizer.pop_ignore_padding(options.chunk);
                            trivia.assign_prefix = next_prefix;
                            expression = match self.parse_expression(options) {
                                Ok((expression_node, _)) => Some(expression_node),
                                Err(err) => return Err(err),
                            };
                        }
                        _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Declaration is not valid."), SourceLocation::default()))
                    }
                    declarations.push(DeclarationNode::new(name.to_owned(), expression, trivia))
                }
                Comma => {
                    self.tokenizer.pop_ignore_whitespace(options.chunk);
                }
                Newline => {
                    self.tokenizer.pop_ignore_whitespace(options.chunk);
                    options.chunk.index -=1;
                    break;
                }
                Semicolon => {
                    self.tokenizer.pop_ignore_whitespace(options.chunk);
                    options.chunk.index -=1;
                    break;
                }
                _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Declaration does not end correctly."), SourceLocation::default()))
            }
        }

        return Ok((Statement::Declaration { kind: kind.clone(), declarations }, None));
    }

    fn parse_return(&mut self, options: &mut ParserOptions) -> Result<(Statement, Option<StatementTerminator>), ParserError> {
        let mut expression_option = None;
        let mut terminator_option = None;
        match self.parse_expression(options) {
            Ok((expression_node, terminator)) => {
                expression_option = Some(expression_node);
                terminator_option = terminator;
            }
            Err(_) => {}
        };

        return Ok((Statement::Return { expression: expression_option }, terminator_option));
    }

    fn parse_import(&mut self, options: &mut ParserOptions) -> Result<(Statement, Option<StatementTerminator>), ParserError> {
        let mut trivia = ImportTrivia::new();

        let (declaration_token, declaration_prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
        trivia.declaration_prefix = declaration_prefix;

        let declaration = match declaration_token {
            Token::Identifier(name) => {
                match name == "*" {
                    true => ImportDeclaration::All,
                    false => ImportDeclaration::Single(name.to_owned())
                }
            }
            _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Invalid import."), SourceLocation::default()))
        };

        let mut alias = None;
        let source;

        loop {
            self.tokenizer.pop_ignore_padding(options.chunk);
            let (next_token, next_prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
            match next_token {
                Token::Keyword(Keyword::As) => {
                    self.tokenizer.pop_ignore_padding(options.chunk);
                    if alias.is_some() {
                        return Err(ParserError::new(ParserErrorKind::Syntax, format!("Invalid import."), SourceLocation::default()));
                    }
                    trivia.as_prefix = next_prefix;
                    let (identifier_token, identifier_prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
                    match identifier_token {
                        Token::Identifier(ref name) => {
                            trivia.alias_prefix = identifier_prefix;
                            alias = Some(name.to_owned());
                        }
                        _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Invalid import."), SourceLocation::default()))
                    }
                }
                Token::Keyword(Keyword::From) => {
                    self.tokenizer.pop_ignore_padding(options.chunk);
                    trivia.from_prefix = next_prefix;
                    let (source_token, source_prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
                    self.tokenizer.pop_ignore_padding(options.chunk);
                    match source_token {
                        Token::Literal(Literal::String(ref name, ref quote)) => {
                            trivia.source_prefix = source_prefix;
                            trivia.quote_kind = quote.clone();
                            source = Some(name.to_owned());
                        }
                        _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Invalid import."), SourceLocation::default()))
                    };
                    break;
                }
                _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Invalid import."), SourceLocation::default()))
            }
        }

        //TODO: Cleanup, should this be here?
        let import = source.unwrap();
        self.requires.push(import.clone());

        return Ok((Statement::Import { source: import, alias, declaration, trivia }, None));
    }

    fn parse_keyword(&mut self, options: &mut ParserOptions, keyword: Keyword) -> Result<(Statement, Option<StatementTerminator>), ParserError> {
        use self::Keyword::*;

        return match keyword {
            Return => {
                self.tokenizer.pop_ignore_padding(options.chunk);
                self.parse_return(options)
            }
            Declaration(kind) => {
                self.tokenizer.pop_ignore_padding(options.chunk);
                self.parse_declaration(options, &kind)
            }
            Import => {
                self.tokenizer.pop_ignore_padding(options.chunk);
                self.parse_import(options)
            }
            _ => Err(ParserError::new(ParserErrorKind::Syntax, format!("Statement keyword '{:?}' not found.", keyword), SourceLocation::default()))
        };
    }


    fn parse_statement(&mut self, options: &mut ParserOptions) -> Result<StatementNode, ParserError> {
        use self::Token::*;

        let mut trivia = NodeTrivia::new();
        let mut statement_option: Option<Statement> = None;
        let mut terminator_option: Option<StatementTerminator> = None;

        let (token, prefix) = self.tokenizer.peek_ignore_padding(options.chunk);
        trivia.prefix = prefix;
        match token {
            Keyword(keyword) => {
                match self.parse_keyword(options, keyword) {
                    Ok((statement, terminator)) => {
                        statement_option = Some(statement);
                        terminator_option = terminator;
                    }
                    Err(_) => {}
                }
            }
            _ => {
                statement_option = None;
            }
        }
        if statement_option.is_none() {
            match self.parse_expression(options) {
                Ok((mut expression_node, terminator)) => {
                    expression_node.trivia.prefix = String::new();
                    statement_option = Some(Statement::Expression { expression: expression_node });
                    terminator_option = terminator;
                }
                Err(err) => return Err(err)
            }
        }
        if terminator_option.is_none() {
            loop {
                let token = self.tokenizer.peek_token(options.chunk).unwrap();
                 match token {
                    Whitespace(count) => {
                        for _ in 0..count {
                            trivia.suffix += " ";
                        }
                        self.tokenizer.pop_token(options.chunk);
                    }
                    Newline => {
                        self.tokenizer.pop_token(options.chunk);
                        let (next_proper_token, pad) = self.tokenizer.peek_ignore_whitespace(options.chunk);

                        if next_proper_token == Newline || next_proper_token == Semicolon {
                            trivia.suffix += "\n";
                        } else {
                            terminator_option = Some(StatementTerminator::Newline);
                            break;
                        }
                    }
                    Semicolon => {
                        terminator_option = Some(StatementTerminator::Semicolon);
                        self.tokenizer.pop_token(options.chunk);
                        break;
                    }
                    BraceClose => {
                        terminator_option = Some(StatementTerminator::Block);
                        self.tokenizer.pop_token(options.chunk);
                        break;
                    }
                    _ => return Err(ParserError::new(ParserErrorKind::Syntax, format!("Statement '{:?}' is terminated with token '{:?}'", statement_option, token), SourceLocation::default())),
                }
            }
        }

        let mut node = StatementNode::new(statement_option.unwrap(), trivia, terminator_option.unwrap());
        self.apply_plugin(options, PluginPass::StatementNodeEmit(&mut node));
        return Ok(node);
    }
}
