use crate::span::Span;

use self::{token::{Token, TokenKind}, cursor::Cursor};

pub mod cursor;
pub mod token;

pub trait Lexer {
    fn bump_token(&mut self) -> Option<Token>;
    fn number(&mut self) -> Option<TokenKind>;
    fn ident_or_kw(&mut self) -> Option<TokenKind>;
    fn peek(&self) -> Option<char>;
    fn peek_n(&self, n: usize) -> Option<char>;
    fn is_eof(&self) -> bool {
        self.peek().is_none()
    }
    fn span(&self) -> Span;
    fn reset_span(&mut self);
}

pub fn tokeninze_with<L: Lexer>(mut lexer: L) -> impl Iterator<Item = Token> {
    std::iter::from_fn(move || lexer.bump_token())
}

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token> + 'a {
    tokeninze_with(Cursor::<'a>::new(input))
}
