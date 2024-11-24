use crate::types::program::{Program, ProgramStatement};

use super::{Coordinate, ToDigital};

impl ToDigital for Program {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> Vec<Coordinate> {
        for statement in &self.statements {
            statement.convert_to_digital(circuit);
        }

        vec![]
    }
}

impl ToDigital for ProgramStatement {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> Vec<Coordinate> {
        match self {
            ProgramStatement::VariableDefinitions(definitions) => {
                definitions.convert_to_digital(circuit);
            }
            ProgramStatement::Module(_) => {}
        }

        vec![]
    }
}
