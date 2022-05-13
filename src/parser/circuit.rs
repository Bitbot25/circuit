use std::cell::RefCell;
use std::fs::File;
use std::iter::Peekable;
use std::ops::Deref;

use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::{
    AbstractExpression, AbstractStatement, Ast, BinaryOp, Block, Call, FunctionDecl, Unary,
};
use crate::span::{FileIndex, Span, SpanGuard, SpanStack};

use super::ast::PropertyAccess;

#[derive(Debug)]
pub struct CircuitError {
    details: String,
    span: Span,
}

impl CircuitError {
    pub fn details(&self) -> &String {
        &self.details
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

// FIXME: Fix spans
pub struct Circuit<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    stack: SpanStack,
}

// TODO: Add peek_kind and bump_kind functions.
impl<I: Iterator<Item = Token>> Circuit<I> {
    pub fn new(tokens: I) -> Circuit<I> {
        Circuit {
            tokens: tokens.peekable(),
            stack: SpanStack::new(),
        }
    }

    fn bump(&mut self) -> Option<Token> {
        let tok = self.tokens.next();
        if let Some(tok) = &tok {
            for span in &mut *self.stack.internal_stack_mut() {
                unsafe {
                    println!("set span to {:?}", tok.span.1);
                    (**span).1 = tok.span.1;
                }
            }
        }
        tok
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn spanned(&mut self) -> SpanGuard {
        // Rust is really annoying sometimes...
        let stack = self.stack.internal_stack();
        let opt_last = stack.last();
        match opt_last {
            Some(last) => {
                let index = unsafe { (**last).1 };
                self.stack.push(Span(index, index))
            },
            None => {
                drop(stack);
                let span = self.peek().map(|tok| tok.span.clone()).unwrap_or_default();
                self.stack.push(span)
            }
        }
    }

    pub fn expect(&mut self, kind: TokenKind, error_msg: &'static str) -> Result<(), CircuitError> {
        let span = self.spanned();
        let tok = self.bump().ok_or_else(|| CircuitError {
            details: error_msg.to_string(),
            span: span.clone(),
        })?;
        if tok.kind != kind {
            Err(CircuitError {
                details: error_msg.to_string(),
                span: span.clone(),
            })
        } else {
            Ok(())
        }
    }

    pub fn block(&mut self) -> Result<Block, CircuitError> {
        let mut statements = Vec::new();
        while match self.peek().map(|tok| &tok.kind) {
            Some(&TokenKind::RBrace) | None => false,
            _ => true,
        } {
            statements.push(self.declaration()?);
        }

        self.expect(TokenKind::RBrace, "Expected RBrace after block.")?;
        Ok(Block { statements })
    }

    pub fn parse(&mut self) -> Result<Ast, CircuitError> {
        let mut statements = Vec::new();
        while let Some(_) = self.peek() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    pub fn statement(&mut self) -> Result<AbstractStatement, CircuitError> {
        self.expr_statement()
    }

    pub fn expr_statement(&mut self) -> Result<AbstractStatement, CircuitError> {
        let expr = self.expression()?;
        self.expect(TokenKind::Semi, "Expected semicolon after expression.")?;
        Ok(AbstractStatement::Expr(expr))
    }

    pub fn declaration(&mut self) -> Result<AbstractStatement, CircuitError> {
        match self.peek().map(|tok| &tok.kind) {
            Some(TokenKind::Function) => self.function_declaration(),
            _ => self.statement(),
        }
    }

    pub fn function_declaration(&mut self) -> Result<AbstractStatement, CircuitError> {
        self.expect(TokenKind::Function, "Expected function keyword!")?;
        let ident = self.bump().ok_or_else(|| CircuitError {
            details: "Expected an identifier but got EOF.".to_string(),
            span: todo!(),
        })?;
        if let TokenKind::Ident(_) = ident.kind {
            self.expect(
                TokenKind::LParen,
                "Expected a left parenthesis before declaring function arguments.",
            )?;
            let mut arguments = Vec::new();
            if self.peek().map(|tok| &tok.kind) != Some(&TokenKind::RParen) {
                loop {
                    let token = self.bump().unwrap();
                    if let TokenKind::Ident(_) = token.kind {
                        arguments.push(token);
                    } else {
                        return Err(CircuitError {
                            details: format!("'{:?}' cannot be used as a parameter.", token.kind),
                            span: todo!(),
                        });
                    }

                    match self.peek().map(|tok| &tok.kind) {
                        Some(TokenKind::Comma) => {
                            self.bump();
                        }
                        _ => break,
                    }
                }
            }
            self.expect(
                TokenKind::RParen,
                "Expected a right parenthesis after declaring function arguments.",
            )?;
            self.expect(
                TokenKind::LBrace,
                "Expected a left brace before function body.",
            )?;
            Ok(AbstractStatement::FunctionDecl(FunctionDecl {
                ident,
                arguments,
                body: self.block()?,
            }))
        } else {
            Err(CircuitError {
                details: "Expected an identifier.".to_string(),
                span: todo!(),
            })
        }
    }

    pub fn expression(&mut self) -> Result<AbstractExpression, CircuitError> {
        self.addition()
    }

    pub fn addition(&mut self) -> Result<AbstractExpression, CircuitError> {
        let mut lhs = self.multiplication()?;
        loop {
            let next_tok = self.peek().map(|tok| &tok.kind);
            if let Some(TokenKind::Plus | TokenKind::Minus) = next_tok {
                let operator = self.bump().unwrap();
                let rhs = self.multiplication()?;
                lhs = AbstractExpression::BinaryOp(BinaryOp {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    pub fn multiplication(&mut self) -> Result<AbstractExpression, CircuitError> {
        let mut lhs = self.unary()?;
        loop {
            let next_tok = self.peek().map(|tok| &tok.kind);
            if let Some(TokenKind::Star | TokenKind::Slash) = next_tok {
                let operator = self.bump().unwrap();
                let rhs = self.unary()?;
                lhs = AbstractExpression::BinaryOp(BinaryOp {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    pub fn unary(&mut self) -> Result<AbstractExpression, CircuitError> {
        match self.peek().map(|tok| &tok.kind) {
            Some(t) if *t == TokenKind::Bang || *t == TokenKind::Minus => {
                let op = self.bump().unwrap();
                let right = self.unary()?;
                Ok(AbstractExpression::Unary(Unary {
                    op,
                    expr: Box::new(right),
                }))
            }
            _ => self.call(),
        }
    }

    pub fn call(&mut self) -> Result<AbstractExpression, CircuitError> {
        let mut expr = self.property_access()?;
        while let Some(TokenKind::LParen) = self.peek().map(|tok| &tok.kind) {
            self.bump();
            let mut arguments = Vec::new();

            if self.peek().map(|tok| &tok.kind) != Some(&TokenKind::RParen) {
                loop {
                    arguments.push(self.expression()?);
                    match self.peek().map(|tok| &tok.kind) {
                        Some(TokenKind::Comma) => {
                            self.bump();
                        }
                        _ => break,
                    }
                }
            }
            expr = AbstractExpression::Call(Call {
                expr: Box::new(expr),
                arguments,
            });
            self.expect(
                TokenKind::RParen,
                "Expected a right parenthesis after call arguments.",
            )?;
        }
        Ok(expr)
    }

    pub fn property_access(&mut self) -> Result<AbstractExpression, CircuitError> {
        let mut expr = self.primary()?;
        while let Some(TokenKind::Dot) = self.peek().map(|tok| &tok.kind) {
            self.bump();
            expr = AbstractExpression::PropertyAccess(PropertyAccess {
                obj: Box::new(expr),
                property: self.ident()?,
            });
        }
        Ok(expr)
    }

    pub fn ident(&mut self) -> Result<Token, CircuitError> {
        let tok = self.bump().ok_or_else(|| CircuitError {
            details: "Expected identifier".to_string(),
            span: todo!(),
        })?;
        match tok.kind {
            TokenKind::Ident(_) => Ok(tok),
            _ => Err(CircuitError {
                details: "Expected identifier".to_string(),
                span: todo!(),
            }),
        }
    }

    pub fn primary(&mut self) -> Result<AbstractExpression, CircuitError> {
        let tok = self.bump().ok_or_else(|| CircuitError {
            details: "Expected a primary value".to_string(),
            span: todo!(),
        })?;
        match tok.kind {
            TokenKind::LParen => {
                let expr = self.expression()?;
                match self.bump().map(|tok| tok.kind) {
                    Some(TokenKind::RParen) => Ok(AbstractExpression::Grouping(Box::new(expr))),
                    _ => Err(CircuitError {
                        details: String::from("Expected left parenthesis after grouping."),
                        span: todo!(),
                    }),
                }
            }
            TokenKind::LBrace => Ok(AbstractExpression::BlockExpression(self.block()?)),
            TokenKind::UInt(_) | TokenKind::String(_) | TokenKind::True | TokenKind::False => {
                Ok(AbstractExpression::Lit(tok))
            }
            // FIXME: Add new property_access parser that parses just identifier or a property access.
            TokenKind::Ident(_) => Ok(AbstractExpression::CEnvPropertyAccess(tok)),
            _ => Err(CircuitError {
                details: format!("Unexpected token: {:?}", tok.kind),
                span: todo!(),
            }),
        }
    }
}
