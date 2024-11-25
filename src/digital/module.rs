use crate::types::module::{ExternalModule, Module};

use super::{Circuit, CircuitModule, DigitalData, ToDigital};

impl ToDigital for Module {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> super::DigitalData {
        circuit.modules.push(CircuitModule::Internal(self.clone()));

        DigitalData::Empty
    }
}
impl ToDigital for ExternalModule {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> super::DigitalData {
        circuit.modules.push(CircuitModule::External(self.clone()));

        DigitalData::Empty
    }
}
