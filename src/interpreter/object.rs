use std::fmt::Debug;

use super::{environment::Environment, prototype::Prototype};
use crate::parser::ast::{AbstractExpression, AbstractStatement, Ast, Block};

pub struct CircuitObject {
    env: Environment,
    prototype: Prototype,
}
