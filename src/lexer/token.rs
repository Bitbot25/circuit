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
    [+] => { $crate::token::TokenKind::Plus };
    [-] => { $crate::token::TokenKind::Minus };
    [*] => { $crate::token::TokenKind::Star };
    [/] => { $crate::token::TokenKind::Slash };
    [true] => { $crate::token::TokenKind::True };
    [false] => { $crate::token::TokenKind::False };
    [if] => { $crate::token::TokenKind::If };
    [for] => { $crate::token::TokenKind::For };
    [return] => { $crate::token::TokenKind::Return };
    [function] => { $crate::token::TokenKind::Function };
    [=] => { $crate::token::TokenKind::Eq }; 
    [==] => { $crate::token::TokenKind::EqEq };
    [!] => { $crate::token::TokenKind::Bang };
    [!=] => { $crate::token::TokenKind::BangEq };
    [.] => { $crate::token::TokenKind::Dot };
    [;] => { $crate::token::TokenKind::Semi };
    [,] => { $crate::token::TokenKind::Comma };
}

