use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum MacroExpression {
    Integer(u32),
    Variable(String),
    UnaryOp(MacroUnaryOp),
    BinaryOp(MacroBinaryOp),
}

#[derive(Debug, Clone)]
pub enum MacroUnaryOp {
    Neg(Arc<MacroExpression>),
}

#[derive(Debug, Clone)]
pub enum MacroBinaryOp {
    Add(Arc<MacroExpression>, Arc<MacroExpression>),
    Sub(Arc<MacroExpression>, Arc<MacroExpression>),
    Mul(Arc<MacroExpression>, Arc<MacroExpression>),
    Div(Arc<MacroExpression>, Arc<MacroExpression>),
    Mod(Arc<MacroExpression>, Arc<MacroExpression>),
}

pub trait ExpandMacro {
    fn expand(&self) -> String;
}
