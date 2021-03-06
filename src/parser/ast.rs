use crate::lexer::token::Token;

pub type Ast = Vec<AbstractStatement>;

#[derive(Debug)]
pub enum AbstractStatement {
    Expr(AbstractExpression),
    // A block can appear without an expression statement
    Block(Block),
    FunctionDecl(FunctionDecl),
    Return(AbstractExpression),
}

#[derive(Debug)]
pub enum AbstractExpression {
    Grouping(Box<AbstractExpression>),
    Binary(Binary),
    Literal(AbstractLiteral),
    // Inline block expression
    // Example:
    // my_fn({ return 2+2; })
    BlockExpression(Block),
    PropertyAccess(PropertyAccess),
    Unary(Unary),
    Call(Call),
}

#[derive(Debug)]
pub enum AbstractLiteral {
    UInt(u64),
    String(String),
}

#[derive(Debug)]
pub struct PropertyAccess {
    // None means the current environment
    pub obj: Option<Box<AbstractExpression>>,
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
    pub args: Vec<AbstractExpression>,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Ast,
}

#[derive(Debug)]
pub struct Binary {
    pub operator: Token,
    pub lhs: Box<AbstractExpression>,
    pub rhs: Box<AbstractExpression>,
}