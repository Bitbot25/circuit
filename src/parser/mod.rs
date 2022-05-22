use std::ops::Range;

use crate::lexer::{
    token::{Token, TokenKind},
    TokenStream,
};
use crate::span::Span;

pub mod ast;
pub mod parse;

pub struct ParseStream<'src> {
    tokens: TokenStream,
    lexeme: &'src str,
}

impl<'src> ParseStream<'src> {
    pub fn new(tokens: TokenStream, lexeme: &'src str) -> ParseStream<'src> {
        ParseStream { tokens, lexeme }
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.tokens.peek()
    }

    pub fn peeks(&mut self, kind: TokenKind) -> bool {
        let tok = self.tokens.peek();
        match tok {
            Some(tok) => tok.kind == kind,
            None => false,
        }
    }

    pub fn peeks_any<const N: usize>(&mut self, kinds: [TokenKind; N]) -> bool {
        let tok = self.tokens.peek();
        match tok {
            Some(tok) => {
                for kind in kinds {
                    if tok.kind == kind {
                        return true;
                    }
                }   
                false
            }
            None => false,
        }   
    }

    pub fn get(&mut self, kind: TokenKind) -> Option<Token> {
        let tok = self.tokens.peek()?;
        if tok.kind == kind {
            self.tokens.next();
            Some(tok)
        } else {
            None
        }
    }

    pub fn gets(&mut self, kind: TokenKind) -> bool {
        self.get(kind).is_some()
    }

    pub fn get_any<const N: usize>(&mut self, kinds: [TokenKind; N]) -> Option<Token> {
        let tok = self.tokens.peek()?;
        for kind in kinds {
            if tok.kind == kind {
                self.tokens.next();
                return Some(tok);
            }
        }
        None
    }

    pub fn expect(&mut self, kind: TokenKind, err: &'static str) -> Result<(), &'static str> {
        let tok = self.tokens.next().ok_or(err)?;
        if tok.kind == kind {
            Ok(())
        } else {
            Err(err)
        }
    }

    pub fn src(&self, range: Range<usize>) -> &'src str {
        &self.lexeme[range]
    }

    pub fn src_from_span(&self, span: Span) -> &'src str {
        &self.lexeme[span.0.index..span.1.index]
    }
}

impl<'src> Iterator for ParseStream<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
    }
}