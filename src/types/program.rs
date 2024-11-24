use crate::parser::ParserModuleVariableData;

use super::{module::Module, variable_definition::VariableDefinitions};

#[derive(Debug)]
pub enum ProgramStatement {
    VariableDefinitions(VariableDefinitions),
    Module(Module),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<ProgramStatement>,

    pub inputs: Vec<ParserModuleVariableData>,
    pub outputs: Vec<ParserModuleVariableData>,
}
