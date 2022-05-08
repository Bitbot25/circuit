use std::iter::Peekable;

use crate::parser::ast::{Ast, AbstractStatement, AbstractExpression, BinaryOp, Unary, Call, FunctionDecl, Block};
use crate::span::{Span, RichIndex};
use crate::lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub struct CircuitError {
    details: String,
    span: Span,
}

// TODO: Implement a Span for all the expressions. So we actually have a Span here as a field that we can extend from the spans of tokens.
pub struct Circuit<I: Iterator<Item=Token>> {
    tokens: Peekable<I>,
    span: Span,
}

// TODO: Add peek_kind and bump_kind functions.
impl<I: Iterator<Item=Token>> Circuit<I> {
    pub fn new(tokens: I) -> Circuit<I> {
        Circuit {
            tokens: tokens.peekable(),
            span: Span {
                begin: RichIndex { index: 0, line: 0, column: 0, },
                end: RichIndex { index: 0, line: 0, column: 0 },
            }
        }
    }

    fn bump(&mut self) -> Option<Token> {
        let tok = self.tokens.next();
        if let Some(tok) = &tok {
            self.span.end = tok.span.end;
        }
        tok
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn expect(&mut self, kind: TokenKind, error_msg: &'static str) -> Result<(), CircuitError> {
        let tok = self.bump().ok_or_else(|| CircuitError { details: error_msg.to_string(), span: self.span })?;
        if tok.kind != kind {
            Err(CircuitError { details: error_msg.to_string(), span: self.span })
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
        let ident = self.bump().ok_or_else(|| CircuitError { details: "Expected an identifier but got EOF.".to_string(), span: self.span })?;
        if let TokenKind::Ident(_) = ident.kind {
            self.expect(TokenKind::LParen, "Expected a left parenthesis before declaring function arguments.")?;
            let mut arguments = Vec::new();
            if self.peek().map(|tok| &tok.kind) != Some(&TokenKind::RParen) {
                loop {
                    let token = self.bump().unwrap();
                    if let TokenKind::Ident(_) = token.kind {
                        arguments.push(token);
                    } else {
                        return Err(CircuitError { details: format!("'{:?}' cannot be used as a parameter.", token.kind), span: self.span });
                    }

                    match self.peek().map(|tok| &tok.kind) {
                        Some(TokenKind::Comma) => {
                            self.bump();
                        },
                        _ => break
                    }
                }
            }
            self.expect(TokenKind::RParen, "Expected a right parenthesis after declaring function arguments.")?;
            self.expect(TokenKind::LBrace, "Expected a left brace before function body.")?;
            Ok(AbstractStatement::FunctionDecl(FunctionDecl {
                ident,
                arguments,
                body: self.block()?,
            }))
        } else {
            Err(CircuitError { details: "Expected an identifier.".to_string(), span: self.span })
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
                lhs = AbstractExpression::BinaryOp(BinaryOp { operator, lhs: Box::new(lhs), rhs: Box::new(rhs)  })
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
                lhs = AbstractExpression::BinaryOp(BinaryOp { operator, lhs: Box::new(lhs), rhs: Box::new(rhs)  })
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
            },
            _ => self.call(),
        }
    }

    // TODO: Add property_access as a parent of call

    pub fn call(&mut self) -> Result<AbstractExpression, CircuitError> {
        let mut expr = self.primary()?;
        while let Some(TokenKind::LParen) = self.peek().map(|tok| &tok.kind) {
            self.bump();
            let mut arguments = Vec::new();
            
            if self.peek().map(|tok| &tok.kind) != Some(&TokenKind::RParen) {
                loop {
                    arguments.push(self.expression()?);
                    match self.peek().map(|tok| &tok.kind) {
                        Some(TokenKind::Comma) => { self.bump(); }
                        _ => break,
                    }
                }
            }
            expr = AbstractExpression::Call(Call { expr: Box::new(expr), arguments });
            self.expect(TokenKind::RParen, "Expected a right parenthesis after call arguments.")?;
        } 
        Ok(expr)
    }

    pub fn primary(&mut self) -> Result<AbstractExpression, CircuitError> {
        let tok = self.bump().ok_or_else(|| CircuitError { details: "Expected a primary value".to_string(), span: self.span })?;
        self.span.tp_end();
        match tok.kind {
            TokenKind::LParen => {
                let expr = self.expression()?;
                match self.bump().map(|tok| tok.kind) {
                    Some(TokenKind::RParen) => Ok(AbstractExpression::Grouping(Box::new(expr))),
                    _ => Err(CircuitError { details: String::from("Expected left parenthesis after grouping."), span: self.span }) 
                }
            },
            TokenKind::LBrace => Ok(AbstractExpression::BlockExpression(self.block()?)),
            TokenKind::UInt(_) |
            TokenKind::String(_) | 
            TokenKind::True | 
            TokenKind::False => Ok(AbstractExpression::Lit(tok)),
            TokenKind::Ident(_) => Ok(AbstractExpression::Var(tok)),
            _ => Err(CircuitError { details: format!("Unexpected token: {:?}", tok.kind), span: self.span }),
        }
    }
}