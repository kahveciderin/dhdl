use super::expression::ExpressionWithWidth;

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: Option<String>,
    pub value: ExpressionWithWidth,
}
