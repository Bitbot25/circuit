use unicode_xid::UnicodeXID;

use crate::lexer::token::{Token, TokenKind};
use crate::span::*;
use std::iter::Peekable;
use std::str::Chars;

const KEYWORDS: [(&'static str, TokenKind); 4] = [
    ("if", TokenKind::If),
    ("for", TokenKind::For),
    ("function", TokenKind::Function),
    ("return", TokenKind::Return),
];

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
            span: Span(
                FileIndex {
                    index: 0,
                    line: 0,
                    column: 0,
                },
                FileIndex {
                    index: 0,
                    line: 0,
                    column: 0,
                },
            ),
        }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.iter.next();
        if let Some(c) = c {
            self.span.notice(c);
        }
        c
    }

    fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|c| *c)
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

    pub fn bump_token(&mut self) -> Option<Token> {
        self.reset_span();
        let c = self.bump()?;
        if is_whitespace(c) {
            return self.bump_token();
        }

        let token_kind = match c {
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '=' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::NotEq
                } else {
                    TokenKind::Bang
                }
            }
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semi,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            _ if c.is_digit(10) => self.number()?,
            _ if is_symbol_start(c) => self.ident_or_kw()?,
            _ => return None,
        };
        Some(Token {
            span: self.span.clone(),
            kind: token_kind,
        })
    }

    fn ident_or_kw(&mut self) -> Option<TokenKind> {
        self.bump_while(is_symbol_continue);

        let val = self.span_str();

        for (kw, kind) in KEYWORDS {
            if val == kw {
                return Some(kind);
            }
        }

        return Some(TokenKind::Ident(String::from(val)));
    }

    fn number(&mut self) -> Option<TokenKind> {
        // TODO: Add support for signed ints and floats.
        self.bump_while(|c| c.is_digit(10));
        Some(TokenKind::UInt(self.span_str().parse().unwrap()))
    }

    pub fn span(&self) -> Span {
        self.span.clone()
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
