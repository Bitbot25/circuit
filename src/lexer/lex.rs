use unicode_xid::UnicodeXID;

use crate::lexer::token::{Token, TokenKind};
use crate::span::*;
use std::iter::Peekable;
use std::str::Chars;

const KEYWORDS: [(&'static str, TokenKind); 4] = [
    ("if", TokenKind::If),
    ("for", TokenKind::For),
    ("fun", TokenKind::Fun),
    ("return", TokenKind::Return),
];

#[derive(Debug)]
pub struct LexError {
    pub span: Span,
    pub details: String,
}

pub struct Lexer<'a> {
    src: &'a str,
    iter: Peekable<Chars<'a>>,
    span: Span,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            src: input,
            iter: input.chars().peekable(),
            span: Default::default(),
        }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.iter.next();
        c.map(|c| {
            self.span.notice(c);
            c
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.iter.peek().copied()
    }

    fn span_str(&self) -> &'a str {
        &self.src[self.span.0.index..self.span.1.index]
    }

    fn bump_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        // Let chains is unstable so we don't use `while let`.
        while self.peek().is_some() && pred(self.peek().unwrap()) {
            self.bump();
        }
    }

    pub fn token(&mut self) -> Option<Result<Token, LexError>> {
        self.reset_span();
        let c = self.bump()?;
        if is_whitespace(c) {
            return self.token();
        }

        match match c {
            '*' => Ok(TokenKind::Star),
            '/' => Ok(TokenKind::Slash),
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '=' => {
                Ok(if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::Eq
                } else {
                    TokenKind::EqEq
                })
            }
            '!' => {
                Ok(if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::BangEq
                } else {
                    TokenKind::Bang
                })
            }
            '.' => Ok(TokenKind::Dot),
            ',' => Ok(TokenKind::Comma),
            ';' => Ok(TokenKind::Semi),
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            '\"' => self.string(),
            _ if c.is_digit(10) => Ok(self.number()),
            _ if is_symbol_start(c) => Ok(self.ident_or_kw()),
            _ => return None,
        } {
            Ok(kind) => Some(Ok(Token { span: self.span, kind })),
            Err(error) => Some(Err(error)),
        }
    }

    fn string(&mut self) -> Result<TokenKind, LexError> {
        self.bump_while(|c| c != '\"');
        match self.bump() {
            Some('\"') => (),
            _ => return Err(self.error(String::from("Expected quote after string."))), 
        };
        Ok(TokenKind::String)
    }

    fn ident_or_kw(&mut self) -> TokenKind {
        self.bump_while(is_symbol_continue);

        let val = self.span_str();

        for (kw, kind) in KEYWORDS {
            if val == kw {
                return kind;
            }
        }

        TokenKind::Ident
    }

    fn number(&mut self) -> TokenKind {
        // TODO: Add support for signed ints and floats.
        self.bump_while(|c| c.is_digit(10));
        TokenKind::UInt
    }

    fn error(&mut self, details: String) -> LexError {
        LexError { span: self.span, details: details }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    fn reset_span(&mut self) {
        self.span.blip();
    }
}

fn is_symbol_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c)
}

fn is_symbol_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}
