use crate::types::module::{ExternalModule, Module};

use super::{Circuit, CircuitModule, DigitalData, ToDigital};

impl ToDigital for Module {
    fn convert_to_digital(&self, _circuit: &mut Circuit) -> super::DigitalData {
        todo!("convert module to digital")
    }
}
impl ToDigital for ExternalModule {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> super::DigitalData {
        circuit.modules.push(CircuitModule::External(self.clone()));

        DigitalData::Empty
    }
}
