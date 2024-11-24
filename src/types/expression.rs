use std::{collections::HashMap, sync::Arc};

use crate::parser::datatype::KnownBitWidth;

use super::argument::Argument;

#[derive(Debug, Clone)]
pub struct ExpressionWithWidth {
    pub expression: Expression,
    pub width: KnownBitWidth,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(u32),
    Variable(String),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    Extract(Extract),
    Combine(Combine),
    ModuleUse(ModuleUse),
}

#[derive(Debug, Clone)]
pub struct Extract {
    pub expression: Arc<ExpressionWithWidth>,
    pub extract: ExtractInner,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not(Arc<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    And(Arc<Expression>, Arc<Expression>),
    NAnd(Arc<Expression>, Arc<Expression>),
    Or(Arc<Expression>, Arc<Expression>),
    NOr(Arc<Expression>, Arc<Expression>),
    XOr(Arc<Expression>, Arc<Expression>),
    XNOr(Arc<Expression>, Arc<Expression>),
}

#[derive(Debug, Clone)]
pub enum ExtractInner {
    Bit(u32),
    Range(u32, u32),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum Combine {
    Bits(Vec<Expression>),
    Obj(HashMap<String, Expression>),
}

#[derive(Debug, Clone)]
pub struct ModuleUse {
    pub name: String,
    pub arguments: HashMap<String, Argument>,
}
