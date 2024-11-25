
use super::{
    expression::Expression,
    module::{ExternalModule, Module},
    variable_definition::VariableDefinitions,
};

#[derive(Debug, Clone)]
pub enum ProgramStatement {
    VariableDefinitions(VariableDefinitions),
    Module(Module),
    ExternalModule(ExternalModule),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<ProgramStatement>,
}
