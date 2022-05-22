use super::{ast::*, ParseStream};
use crate::lexer::token::{TokenKind, Token};

pub fn literal(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    let tok = stream.get_any([TokenKind::UInt, TokenKind::String]).ok_or("Expected number or string.")?;
    Ok(AbstractExpression::Lit(Literal { tok }))
}

pub fn add(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    let mut expr = mul(stream)?;
    while let Some(operator) = stream.get_any([TokenKind::Plus, TokenKind::Minus]) {
        expr = AbstractExpression::Binary(Binary { operator, lhs: Box::new(expr), rhs: Box::new(mul(stream)?) }); 
    }
    Ok(expr)
}

pub fn mul(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    let mut expr = unary(stream)?;
    while let Some(operator) = stream.get_any([TokenKind::Star, TokenKind::Slash]) {
        expr = AbstractExpression::Binary(Binary { operator, lhs: Box::new(expr), rhs: Box::new(unary(stream)?) });
    }
    Ok(expr)
}

pub fn unary(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    if let Some(op) = stream.get_any([TokenKind::Bang, TokenKind::Minus]) {
        Ok(AbstractExpression::Unary(Unary { op, expr: Box::new(unary(stream)?) }))
    } else {
        property(stream)
    }
}

pub fn property(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    if let Some(init_prop) = stream.get(TokenKind::Ident) {
        let mut expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: None, property: init_prop });
        while let Some(tok) = stream.get_any([TokenKind::Dot, TokenKind::LParen]) {
            match tok.kind {
                TokenKind::Dot => {
                    let property = ident(stream)?;
                    expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: Some(Box::new(expr)), property });
                },
                TokenKind::LParen => {
                    let mut args = vec![];
                    if !stream.peeks(TokenKind::RParen) {
                        loop {
                            // TODO: Change this to expr
                            args.push(property(stream)?);
                            if !stream.gets(TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    stream.expect(TokenKind::RParen, "Expected a closing parentheis ')' after arguments.")?;
                    expr = AbstractExpression::Call(Call { expr: Box::new(expr), args });
                },
                _ => unreachable!(),
            }   
        }
        Ok(expr)
    } else {
        literal(stream)
    }
}

pub fn ident(stream: &mut ParseStream) -> Result<Token, &'static str> {
    stream.get(TokenKind::Ident).ok_or("Expected identifier.")
}

/*pub fn property(&mut self) -> Result<AbstractExpression, CircuitError> {
    match self.peek_kind() {
        Some(TokenKind::Ident(_)) => {
            let init_prop = self.bump().unwrap();
            let mut expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: None, property: init_prop });
            if let Some(TokenKind::Dot) = self.peek_kind() {
                self.bump();

                loop {
                    let property = self.ident()?;

                    expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: Some(Box::new(expr)), property });
                    match self.peek_kind() {
                        Some(TokenKind::Dot) => (),
                        _ => break,
                    }
                }
            }
            Ok(expr)

        },
        _ => self.primary(),
    }
}*/


/*pub fn unary(&mut self) -> Result<AbstractExpression, CircuitError> {
    let span = self.spanned();
    match self.peek_kind() {
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
}*/

/*pub fn call(&mut self) -> Result<AbstractExpression, CircuitError> {
    let mut expr = self.property()?;

    while let Some(TokenKind::LParen) = self.peek_kind() {
        self.bump();
        let mut arguments = Vec::new();

        match self.peek_kind() {
            Some(TokenKind::RParen) => (),
            Some(_) => loop {
                arguments.push(self.expression()?);
                match self.peek_kind() {
                    Some(TokenKind::Comma) => self.bump(),
                    _ => break,
                };
            },
            None => return Err(CircuitError { details: "Expected arguments after lparen.".to_string(), span: Default::default()  })
        }

        expr = AbstractExpression::Call(Call { expr: Box::new(expr), arguments });
        self.expect(TokenKind::RParen, "Expected rparen after call arguments.")?;
    }

    Ok(expr)
}*/

