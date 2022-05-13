use crate::lexer::token::Token;

pub type Ast = Vec<AbstractStatement>;

#[derive(Debug)]
pub enum AbstractStatement {
    Expr(AbstractExpression),
    // A block can appear without an expression statement
    BlockStatement(Block),
    FunctionDecl(FunctionDecl),
    Return(AbstractExpression),
}

#[derive(Debug)]
pub enum AbstractExpression {
    Grouping(Box<AbstractExpression>),
    BinaryOp(BinaryOp),
    Lit(Token),
    // Inline block expression
    // Example:
    // my_fn({ return 2+2; })
    BlockExpression(Block),
    PropertyAccess(PropertyAccess),
    Unary(Unary),
    CEnvPropertyAccess(Token),
    Call(Call),
}

#[derive(Debug)]
pub struct PropertyAccess {
    pub obj: Box<AbstractExpression>,
    pub property: Token,
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Token,
    pub arguments: Vec<Token>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Unary {
    pub op: Token,
    pub expr: Box<AbstractExpression>,
}

#[derive(Debug)]
pub struct Call {
    pub expr: Box<AbstractExpression>,
    pub arguments: Vec<AbstractExpression>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Ast,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub operator: Token,
    pub lhs: Box<AbstractExpression>,
    pub rhs: Box<AbstractExpression>,
}
