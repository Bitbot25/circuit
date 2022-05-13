use crate::span::Span;

use self::{cursor::Lexer, token::Token};

pub mod cursor;
pub mod token;

pub fn tokenize_with<'a>(mut lexer: Lexer<'a>) -> impl Iterator<Item = Token> + 'a {
    std::iter::from_fn(move || lexer.bump_token())
}

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token> + 'a {
    tokenize_with(Lexer::<'a>::new(input))
}
