use crate::lang::{span::Span, token::TokenKind::Def};
use derive_more::Display;

#[derive(Debug, Copy, Display, Clone, PartialEq)]
pub enum TokenKind {
    Def,
    Eq,
    Term,
    Char,
    Int,
    Float,
    String,
    Indent,
    Dedent,
    LBrace,
    RBrace,
    Backslash,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub value: Option<TokenValue>,
}

impl Token {
    pub fn new(span: Span, kind: TokenKind) -> Self {
        Self {
            span,
            kind,
            value: None,
        }
    }
    pub fn of(kind: TokenKind) -> Self {
        Self {
            span: Default::default(),
            kind,
            value: None,
        }
    }
    pub fn with_span(self, span: Span) -> Self {
        Self { span, ..self }
    }
    pub fn with(kind: TokenKind, value: TokenValue) -> Self {
        Self {
            span: Default::default(),
            kind,
            value: Some(value),
        }
    }
    pub fn with_char(kind: TokenKind, value: char) -> Self {
        Self::with(kind, TokenValue::Char(value))
    }
    pub fn with_string(kind: TokenKind, value: String) -> Self {
        Self::with(kind, TokenValue::String(value))
    }
    pub fn with_int(kind: TokenKind, value: i64) -> Self {
        Self::with(kind, TokenValue::Int(value))
    }
    pub fn with_float(kind: TokenKind, value: f64) -> Self {
        Self::with(kind, TokenValue::Float(value))
    }

    pub fn value_char(&self) -> Option<char> {
        match &self.value {
            Some(TokenValue::Char(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn value_string(&self) -> Option<String> {
        match &self.value {
            Some(TokenValue::String(v)) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn value_int(&self) -> Option<i64> {
        match &self.value {
            Some(TokenValue::Int(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn value_float(&self) -> Option<f64> {
        match &self.value {
            Some(TokenValue::Float(v)) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Char(char),
    String(String),
    Int(i64),
    Float(f64),
}
