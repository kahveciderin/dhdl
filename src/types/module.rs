use crate::parser::ParserModuleVariableData;

use super::program::ProgramStatement;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub statements: Vec<ProgramStatement>,

    pub inputs: Vec<ParserModuleVariableData>,
    pub outputs: Vec<ParserModuleVariableData>,
}
