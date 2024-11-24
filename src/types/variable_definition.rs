use super::{decorator::Decorator, expression::ExpressionWithWidth};

#[derive(Debug)]
pub struct VariableDefinition {
    pub name: String,
    pub value: Option<ExpressionWithWidth>,
}

#[derive(Debug)]
pub struct VariableDefinitions {
    pub definitions: Vec<VariableDefinition>,
    pub decorator: Option<Decorator>,
}
