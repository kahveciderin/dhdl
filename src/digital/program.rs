use crate::types::program::{Program, ProgramStatement};

use super::{DigitalData, ToDigital};

impl ToDigital for Program {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> DigitalData {
        for statement in &self.statements {
            statement.convert_to_digital(circuit);
        }

        DigitalData::Empty
    }
}

impl ToDigital for ProgramStatement {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> DigitalData {
        match self {
            ProgramStatement::VariableDefinitions(definitions) => {
                definitions.convert_to_digital(circuit);
            }
            ProgramStatement::Expression(expr) => {
                expr.convert_to_digital(circuit);
            }
            ProgramStatement::Module(module) => {
                module.convert_to_digital(circuit);
            }
            ProgramStatement::ExternalModule(module) => {
                module.convert_to_digital(circuit);
            }
        }

        DigitalData::Empty
    }
}
