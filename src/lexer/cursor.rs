use unicode_xid::UnicodeXID;

use crate::span::*;
use crate::lexer::token::{Token, TokenKind};
use std::str::Chars;

use super::Lexer;

const KEYWORDS: [(&'static str, TokenKind); 4] = [
    ("if", TokenKind::If),
    ("for", TokenKind::For),
    ("function", TokenKind::Function),
    ("return", TokenKind::Return),
];

pub struct Cursor<'a> {
    src: &'a str,
    iter: Chars<'a>,
    span: Span,
    #[cfg(debug_assertions)]
    prev: Option<char>,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            src: input,
            iter: input.chars(),
            span: Span {
                begin: RichIndex {
                    index: 0,
                    line: 0,
                    column: 0,
                },
                end: RichIndex {
                    index: 0,
                    line: 0,
                    column: 0,
                },
            },
            #[cfg(debug_assertions)]
            prev: None,
        }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.iter.next();
        #[cfg(debug_assertions)]
        {
            self.prev = c;
        }
        if c.is_some() {
            self.span.notice(c.unwrap());
        }
        c
    }

    fn span_str(&self) -> &'a str {
        &self.src[self.span.begin.index..self.span.end.index]
    }

    fn bump_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while !self.is_eof() && pred(self.peek().unwrap()) {
            self.bump();
        }
    }

    #[cfg(debug_assertions)]
    fn prev(&self) -> Option<char> {
        self.prev
    }
}

fn is_symbol_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c)
}

fn is_symbol_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        ' ' |
        '\t' |
        '\n'
    )
}

impl<'a> Lexer for Cursor<'a> {
    fn bump_token(&mut self) -> Option<Token> {
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
            },
            '!' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::NotEq
                } else {
                    TokenKind::Bang
                }
            },
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
            span: self.span,
            kind: token_kind,
        })
    }

    fn ident_or_kw(&mut self) -> Option<TokenKind> {
        debug_assert!(self.prev.is_some() && is_symbol_start(self.prev.unwrap()));

        self.bump_while(is_symbol_continue);

        let val = self.span_str();
        
        for (kw, kind) in KEYWORDS {
            if val == kw {
                return Some(kind)
            }
        }

        return Some(TokenKind::Ident(String::from(val)))
    }

    fn number(&mut self) -> Option<TokenKind> {
        // TODO: Add support for signed ints and floats.
        debug_assert!(self.prev.is_some() && self.prev.unwrap().is_digit(10));

        self.bump_while(|c| c.is_digit(10));
        Some(TokenKind::UInt(
            self.span_str().parse().unwrap(),
        ))
    }

    fn span(&self) -> Span {
        self.span
    }

    fn reset_span(&mut self) {
        self.span.tp_end();
    }

    fn peek(&self) -> Option<char> {
        self.iter.clone().next()
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.iter.clone().nth(n)
    }
}