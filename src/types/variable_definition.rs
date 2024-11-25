use super::{decorator::Decorator, expression::ExpressionWithWidth};

#[derive(Debug, Clone)]
pub struct VariableDefinition {
    pub name: String,
    pub value: Option<ExpressionWithWidth>,
}

#[derive(Debug, Clone)]
pub struct VariableDefinitions {
    pub definitions: Vec<VariableDefinition>,
    pub decorator: Option<Decorator>,
}
