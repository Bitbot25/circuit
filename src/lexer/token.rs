use std::fmt::Debug;
use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    True,
    False,
    UInt(u32),
    String(String),
    Ident(String),

    If,
    For,
    Return,
    Function,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Eq,
    Assign,
    NotEq,
    Bang,
    Comma,
    Semi,
    Dot,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    pub(crate) span: Span,
    pub kind: TokenKind,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Token").field(&self.kind).finish()
    }
}