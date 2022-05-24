use super::{ast::*, ParseStream};
use crate::lexer::token::{TokenKind, Token};

// TODO: Make a trait called FallbackParser that has a parse method just like all of these ones. vvvvv

pub fn statement(stream: &mut ParseStream) -> Result<AbstractStatement, &'static str> {
    fun_decl(stream)
} 

pub fn fun_decl(stream: &mut ParseStream) -> Result<AbstractStatement, &'static str> {
    if stream.gets(TokenKind::Fun) {
        let fun_ident = expect_ident(stream)?;
        stream.expect(TokenKind::LParen, "Expected opening parenthesis '('.")?;
        stream.expect(TokenKind::RParen, "Expected closing parenthesis ')'.")?;
        let body = expect_block(stream)?;
        
        Ok(AbstractStatement::FunctionDecl(FunctionDecl { ident: fun_ident, arguments: vec![], body }))
    } else {
        expression_stmt(stream)
    }
}

pub fn expression_stmt(stream: &mut ParseStream) -> Result<AbstractStatement, &'static str> {
    let expr = expression(stream)?;
    stream.expect(TokenKind::Semi, "Expected a semicolon ';' after expression.")?;
    Ok(AbstractStatement::Expr(expr))
}

pub fn expression(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    add(stream)
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
                    let property = expect_ident(stream)?;
                    expr = AbstractExpression::PropertyAccess(PropertyAccess { obj: Some(Box::new(expr)), property });
                },
                TokenKind::LParen => {
                    let mut args = vec![];
                    if !stream.peeks(TokenKind::RParen) {
                        loop {
                            args.push(expression(stream)?);
                            if !stream.gets(TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    stream.expect(TokenKind::RParen, "Expected a closing parenthesis ')' after arguments.")?;
                    expr = AbstractExpression::Call(Call { expr: Box::new(expr), args });
                },
                _ => unreachable!(),
            }   
        }
        Ok(expr)
    } else {
        grouping(stream)
    }
}

pub fn grouping(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    if stream.gets(TokenKind::LParen) {
        let inside = expression(stream)?;
        stream.expect(TokenKind::RParen, "Expected closing parenthesis ')' after expression.")?;
        Ok(AbstractExpression::Grouping(Box::new(inside)))
    } else {
        literal(stream)
    }
}

pub fn literal(stream: &mut ParseStream) -> Result<AbstractExpression, &'static str> {
    let tok = stream.get_any([TokenKind::UInt, TokenKind::String]).ok_or("Expected number or string.")?;
    Ok(AbstractExpression::Literal(match tok.kind {
        TokenKind::UInt => AbstractLiteral::UInt(stream.src_from_span(tok.span).parse().unwrap()),
        TokenKind::String => {
            // FIXME: This is hella sus
            let src = stream.src(tok.span.0.index+1..tok.span.1.index-1);
            AbstractLiteral::String(String::from(src))
        },
        _ => unreachable!(),
    }))
}

pub fn expect_ident(stream: &mut ParseStream) -> Result<Token, &'static str> {
    stream.get(TokenKind::Ident).ok_or("Expected identifier.")
}

pub fn expect_block(stream: &mut ParseStream) -> Result<Block, &'static str> {
    stream.expect(TokenKind::LBrace, "Expected opening brace '{' before block.")?;

    let mut stmts = vec![];
    while !stream.peeks(TokenKind::RBrace) {
        stmts.push(statement(stream)?);
    }

    stream.expect(TokenKind::RBrace, "Expected closing brace '}' after block.")?;

    Ok(Block { stmts })
}