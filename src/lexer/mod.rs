use std::{collections::VecDeque, fmt::Debug};

use crate::{span::Span, iter::TakeErrorsExt};

use self::{lex::{Lexer, LexError}, token::Token};

pub mod lex;
pub mod token;

pub struct TokenStream {
    queue: VecDeque<Token>,
    span: Span,
}

impl TokenStream {
    pub fn new(mut lexer: Lexer) -> Result<TokenStream, LexError> {
        Ok(TokenStream {
            queue: std::iter::from_fn(move || lexer.token()).take_errors().map_err(|errors| errors.into_iter().next().expect("WTF"))?.collect(),
            span: Default::default(),
        })
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.queue.get(0).map(|tok| *tok)
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl Debug for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenStream ")?;
        f.debug_list()
            .entries(&self.queue)
            .finish()
    }
}

pub fn tokenize(input: &str) -> Result<TokenStream, LexError> {
    TokenStream::new(Lexer::new(input))
}
