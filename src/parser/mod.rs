use crate::lexer::{
    token::{Token, TokenKind},
    TokenStream,
};


pub mod ast;
pub mod circuit;

pub struct ParseStream {
    tokens: TokenStream,
}

impl ParseStream {
    pub fn new(tokens: TokenStream) -> ParseStream {
        ParseStream { tokens }
    }

    pub fn peek(&mut self, kind: TokenKind) -> bool {
        let tok = self.tokens.peek();
        match tok {
            Some(tok) => tok.kind == kind,
            None => false,
        }
    }

    pub fn peek_any<const N: usize>(&mut self, kinds: [TokenKind; N]) -> bool {
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

    pub fn consume(&mut self, kind: TokenKind) -> Option<Token> {
        let tok = self.tokens.peek()?;
        if tok.kind == kind {
            self.tokens.next();
            Some(tok)
        } else {
            None
        }
    }

    pub fn consume_any<const N: usize>(&mut self, kinds: [TokenKind; N]) -> Option<Token> {
        let tok = self.tokens.peek()?;
        for kind in kinds {
            if tok.kind == kind {
                self.tokens.next();
                return Some(tok);
            }
        }
        None
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
