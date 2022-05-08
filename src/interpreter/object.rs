use std::fmt::Debug;

use crate::parser::ast::{Ast, AbstractStatement, AbstractExpression, Block};
use super::{prototype::Prototype, environment::Environment};

pub struct CircuitObject {
    env: Environment,
}