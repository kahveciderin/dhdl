use crate::{
    digital::{CircuitVariable, Coordinate, Entry, EntryValue, VisualElement, Wire},
    parser::datatype::KnownBitWidth,
    types::{decorator::Decorator, variable_definition::VariableDefinitions},
};

use super::{Circuit, DigitalData, ToDigital};

pub fn cast_value(value: DigitalData, to: KnownBitWidth, circuit: &mut Circuit) -> Coordinate {
    let from = value.get_size();
    let to = to.get_size();

    if from > to {
        let coordinate = Coordinate::next();

        circuit.visual_elements.push(VisualElement {
            name: String::from("Splitter"),
            attributes: vec![
                Entry {
                    name: String::from("Input Splitting"),
                    value: EntryValue::String(from.to_string()),
                },
                Entry {
                    name: String::from("Output Splitting"),
                    value: EntryValue::String(to.to_string()),
                },
            ],
            position: coordinate.clone(),
        });

        circuit.wires.push(Wire {
            start: value.get_position().clone(),
            end: coordinate.clone(),
        });

        coordinate.add(20, 0)
    } else if from < to {
        let constant_coordinate = Coordinate::next();

        circuit.visual_elements.push(VisualElement {
            name: String::from("Const"),
            attributes: vec![
                Entry {
                    name: String::from("Value"),
                    value: EntryValue::Long(0),
                },
                Entry {
                    name: String::from("Bits"),
                    value: EntryValue::Integer((to - from) as i32),
                },
            ],
            position: constant_coordinate.clone(),
        });

        let coordinate = Coordinate::next();

        circuit.visual_elements.push(VisualElement {
            name: String::from("Splitter"),
            attributes: vec![
                Entry {
                    name: String::from("Input Splitting"),
                    value: EntryValue::String(from.to_string() + ", " + &(to - from).to_string()),
                },
                Entry {
                    name: String::from("Output Splitting"),
                    value: EntryValue::String(to.to_string()),
                },
            ],
            position: coordinate.clone(),
        });

        circuit.wires.push(Wire {
            start: constant_coordinate.clone(),
            end: coordinate.add(0, 20),
        });
        circuit.wires.push(Wire {
            start: value.get_position().clone(),
            end: coordinate.clone(),
        });

        coordinate.add(20, 0)
    } else {
        value.get_position()
    }
}

impl ToDigital for VariableDefinitions {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> DigitalData {
        match &self.decorator {
            None => {
                for def in self.definitions.iter() {
                    if let Some(expression) = &def.value {
                        let data = expression.convert_to_digital(circuit);

                        circuit.add_variable(CircuitVariable {
                            name: def.name.clone(),
                            data,
                        });
                    } else {
                        panic!("Variable {} has no value", def.name);
                    }
                }
            }
            Some(decorator) => match decorator {
                Decorator::In(bits) => {
                    for def in self.definitions.iter() {
                        let coordinate = Coordinate::next();

                        if circuit.is_top() {
                            circuit.visual_elements.push(VisualElement {
                                name: String::from("In"),
                                attributes: vec![
                                    Entry {
                                        name: String::from("Label"),
                                        value: EntryValue::String(def.name.clone()),
                                    },
                                    Entry {
                                        name: String::from("Bits"),
                                        value: EntryValue::Integer(*bits as i32),
                                    },
                                ],
                                position: coordinate.clone(),
                            });
                            circuit.add_variable(CircuitVariable {
                                name: def.name.clone(),
                                data: DigitalData::Wire(*bits, coordinate.clone()),
                            });
                        }
                    }
                }
                Decorator::Out(bits) => {
                    for def in self.definitions.iter() {
                        let coordinate = Coordinate::next();
                        if let Some(expression) = &def.value {
                            let input_wire_position = expression.convert_to_digital(circuit);

                            let target_width = if let Some(bits) = bits {
                                KnownBitWidth::Fixed(*bits)
                            } else {
                                KnownBitWidth::Fixed(input_wire_position.get_size())
                            };

                            let casted_value =
                                cast_value(input_wire_position, target_width.clone(), circuit);

                            if let KnownBitWidth::Fixed(target_width_number) = target_width {
                                if circuit.is_top() {
                                    circuit.visual_elements.push(VisualElement {
                                        name: String::from("Out"),
                                        attributes: vec![
                                            Entry {
                                                name: String::from("Label"),
                                                value: EntryValue::String(def.name.clone()),
                                            },
                                            Entry {
                                                name: String::from("Bits"),
                                                value: EntryValue::Integer(
                                                    target_width_number as i32,
                                                ),
                                            },
                                        ],
                                        position: coordinate.clone(),
                                    });

                                    circuit.wires.push(Wire {
                                        start: casted_value.clone(),
                                        end: coordinate.clone(),
                                    });
                                }
                                circuit.add_variable(CircuitVariable {
                                    name: def.name.clone(),
                                    data: DigitalData::Wire(target_width_number, casted_value),
                                });
                            } else {
                                panic!("b: Output variable {} has no value", def.name);
                            }
                        } else {
                            panic!("c: Output variable {} has no value", def.name);
                        }
                    }
                }
            },
        }

        DigitalData::Empty
    }
}
