use std::{collections::HashMap, sync::Arc};

use crate::parser::{
    datatype::{GetBitWidth, KnownBitWidth},
    ParserState,
};

use super::argument::Argument;

#[derive(Debug, Clone)]
pub struct ExpressionWithWidth {
    pub expression: Expression,
    pub width: KnownBitWidth,
}

impl ExpressionWithWidth {
    pub fn new(expression: Expression, state: &ParserState) -> Self {
        Self {
            width: expression.get_bit_width(state),
            expression,
        }
    }
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
    String(String),
}

#[derive(Debug, Clone)]
pub struct Extract {
    pub expression: Arc<ExpressionWithWidth>,
    pub extract: ExtractInner,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not(Arc<ExpressionWithWidth>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    And(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),
    NAnd(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),
    Or(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),
    NOr(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),
    XOr(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),
    XNOr(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),

    Multiplex(Arc<ExpressionWithWidth>, Arc<ExpressionWithWidth>),
}

#[derive(Debug, Clone)]
pub enum ExtractInner {
    Bit(u32),
    Range(u32, u32),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum Combine {
    Bits(Vec<ExpressionWithWidth>),
    Obj(HashMap<String, Expression>),
}

#[derive(Debug, Clone)]
pub struct ModuleUse {
    pub name: String,
    pub arguments: HashMap<String, Argument>,
}
