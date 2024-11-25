use crate::{
    digital::{Coordinate, Entry},
    parser::{datatype::KnownBitWidth, ParserModuleVariableData},
};

use super::program::ProgramStatement;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub statements: Vec<ProgramStatement>,

    pub inputs: Vec<ParserModuleVariableData>,
    pub outputs: Vec<ParserModuleVariableData>,
}

#[derive(Debug, Clone)]
pub struct ExternalModuleVariableData {
    pub name: String,
    pub width: KnownBitWidth,
    pub position: Coordinate,
}

#[derive(Debug)]
pub struct ExternalModule {
    pub name: String,
    pub rename: Option<String>,

    pub attributes: Vec<Entry>,

    pub inputs: Vec<ExternalModuleVariableData>,
    pub outputs: Vec<ExternalModuleVariableData>,
}
