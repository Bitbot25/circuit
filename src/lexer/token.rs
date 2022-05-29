use crate::span::Span;
use std::fmt::Debug;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,

    True,
    False,
    UInt,
    String,
    Ident,

    If,
    For,
    Return,
    Fun,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Eq,
    EqEq,
    Bang,
    BangEq,
    Dot,
    Semi,
    Comma,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Token {
    pub(crate) span: Span,
    pub kind: TokenKind,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::TokenKind::*;
        write!(
            f,
            "Token[{}]",
            match &self.kind {
                Plus => "+",
                Minus => "-",
                Star => "*",
                Slash => "/",

                True => "true",
                False => "false",
                UInt => "<uint>",
                String => "<string>",
                Ident => "<ident>",

                If => "if",
                For => "for",
                Return => "return",
                Fun => "fun",

                LParen => "(",
                RParen => ")",
                LBrace => "{",
                RBrace => "}",

                Eq => "=",
                EqEq => "==",
                Bang => "!",
                BangEq => "!=",
                Dot => ".",
                Semi => ";",
                Comma => ",",
            }
        )
    }
}

#[macro_export]
macro_rules! Tok {
    [+] => { $crate::lexer::token::TokenKind::Plus };
    [-] => { $crate::lexer::token::TokenKind::Minus };
    [*] => { $crate::lexer::token::TokenKind::Star };
    [/] => { $crate::lexer::token::TokenKind::Slash };
    [true] => { $crate::lexer::token::TokenKind::True };
    [false] => { $crate::lexer::token::TokenKind::False };
    [if] => { $crate::lexer::token::TokenKind::If };
    [for] => { $crate::lexer::token::TokenKind::For };
    [return] => { $crate::lexer::token::TokenKind::Return };
    [function] => { $crate::lexer::token::TokenKind::Function };
    [=] => { $crate::lexer::token::TokenKind::Eq }; 
    [==] => { $crate::lexer::token::TokenKind::EqEq };
    [!] => { $crate::lexer::token::TokenKind::Bang };
    [!=] => { $crate::lexer::token::TokenKind::BangEq };
    [.] => { $crate::lexer::token::TokenKind::Dot };
    [;] => { $crate::lexer::token::TokenKind::Semi };
    [,] => { $crate::lexer::token::TokenKind::Comma };
}

