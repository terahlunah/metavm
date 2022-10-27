use crate::lang::{
    ast::{Ast, Definition},
    lexer::Lexer,
    span::Span,
    token::{Token, TokenKind},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{source} at {:?}", .span)]
pub struct ParseError {
    span: Option<Span>,
    source: ParseErrorKind,
}

impl ParseError {
    pub fn new(span: Option<Span>, source: ParseErrorKind) -> Self {
        Self { span, source }
    }
}

#[derive(Debug, Error)]
pub enum ParseErrorKind {
    #[error("Unexpected end of file")]
    UnexpectedEof,
    #[error("Unexpected char '{0}'")]
    UnexpectedChar(char),
    #[error("Expected a digit")]
    ExpectedDigit,
    #[error("Invalid int '{0}'")]
    ParseInt(String),
    #[error("Invalid float '{0}'")]
    ParseFloat(String),
    #[error("Indentation error")]
    IndentationError,
    #[error("Expected token '{expected}', but got '{got}'")]
    ExpectedToken { expected: TokenKind, got: TokenKind },
    #[error("Expected definition but got '{got}'")]
    ExpectedDefinition { got: TokenKind },
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Self {
            lexer: Lexer::new(source),
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        self.lexer.next()
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let token = self.next()?;
        if token.kind == kind {
            Ok(token)
        } else {
            Err(ParseError::new(
                Some(token.span),
                ParseErrorKind::ExpectedToken {
                    expected: kind,
                    got: token.kind,
                },
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Ast, ParseError> {
        let token = self.next()?;

        let mut definitions = vec![];

        match token.kind {
            TokenKind::Def => definitions.push(self.parse_definition()?),
            _ => {
                return Err(ParseError::new(
                    token.span.into(),
                    ParseErrorKind::ExpectedDefinition { got: token.kind },
                ))
            }
        }

        Ok(Ast {
            imports: vec![],
            definitions,
        })
    }

    fn parse_definition(&mut self) -> Result<Definition, ParseError> {
        todo!()
    }

    fn parse_expr(&mut self) {}
}
