use super::{ast::*, ParseStream};
use crate::lexer::token::{TokenKind, Token};

pub fn literal(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    let tok = stream.consume_any([TokenKind::UInt, TokenKind::String]).ok_or("Expected number or string.")?;
    Ok(AbstractExpression::Lit(Literal { tok }))
}

pub fn property(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    // TODO: Replace `ident` with `call`
    if let Some(init_prop) = stream.consume(TokenKind::Ident) {
        let mut expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: None, property: init_prop });
        while let Some(_) = stream.consume(TokenKind::Dot) {
            let property = ident(stream)?;
            expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: Some(Box::new(expr)), property });
        }
        
        Ok(expr)
    } else {
        literal(stream)
    }
}

pub fn ident(stream: &mut ParseStream) -> Result<Token, &'static str> {
    stream.consume(TokenKind::Ident).ok_or("Expected identifier.")
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

