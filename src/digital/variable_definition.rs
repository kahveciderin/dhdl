use std::cmp::Ordering;

use crate::{
    digital::{CircuitVariable, Coordinate, Entry, EntryValue, VisualElement, Wire},
    parser::datatype::KnownBitWidth,
    types::{decorator::Decorator, variable_definition::VariableDefinitions},
};

use super::{Circuit, DigitalData, ToDigital};

pub fn cast_value(value: DigitalData, to: KnownBitWidth, circuit: &mut Circuit) -> Coordinate {
    let from = value.get_size();
    let to = to.get_size();

    match from.cmp(&to) {
        Ordering::Greater => {
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
        }
        Ordering::Less => {
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
                        value: EntryValue::String(
                            from.to_string() + ", " + &(to - from).to_string(),
                        ),
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
        }
        Ordering::Equal => value.get_position(),
    }
}

impl ToDigital for VariableDefinitions {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> DigitalData {
        match &self.decorator {
            None => {
                // todo: find this variable in circuit, if it exists just connect to it
                for def in self.definitions.iter() {
                    if let Some(expression) = &def.value {
                        let data = expression.convert_to_digital(circuit);

                        let potential_variable = circuit.find_variable(def.name.clone()).cloned();

                        if let Some(potential_variable) = potential_variable {
                            if potential_variable.undefined {
                                let casted_value = cast_value(
                                    data,
                                    KnownBitWidth::Fixed(potential_variable.data.get_size()),
                                    circuit,
                                );
                                circuit.wires.push(Wire {
                                    start: casted_value.clone(),
                                    end: potential_variable.data.get_position().clone(),
                                });
                            } else {
                                panic!("Variable {} already defined", def.name);
                            }
                        } else {
                            circuit.add_variable(CircuitVariable {
                                name: def.name.clone(),
                                data,
                                undefined: false,
                            });
                        }
                    } else {
                        panic!("Variable {} has no value", def.name);
                    }
                }
            }
            Some(decorator) => match decorator {
                Decorator::In(bits, name) => {
                    for def in self.definitions.iter() {
                        let coordinate = Coordinate::next();

                        if circuit.is_top() {
                            circuit.visual_elements.push(VisualElement {
                                name: String::from("In"),
                                attributes: vec![
                                    Entry {
                                        name: String::from("Label"),
                                        value: EntryValue::String(
                                            name.clone().unwrap_or_else(|| def.name.clone()),
                                        ),
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
                                undefined: false,
                            });
                        }
                    }
                }
                Decorator::Clock(freq) => {
                    for def in self.definitions.iter() {
                        let coordinate = Coordinate::next();

                        let mut attributes = vec![Entry {
                            name: String::from("Label"),
                            value: EntryValue::String(def.name.clone()),
                        }];

                        if let Some(freq) = freq {
                            attributes.push(Entry {
                                name: String::from("Frequency"),
                                value: EntryValue::Integer(*freq as i32),
                            });
                            attributes.push(Entry {
                                name: String::from("runRealTime"),
                                value: EntryValue::Boolean(true),
                            })
                        } else {
                            attributes.push(Entry {
                                name: String::from("runRealTime"),
                                value: EntryValue::Boolean(false),
                            })
                        }

                        circuit.visual_elements.push(VisualElement {
                            name: String::from("Clock"),
                            attributes,
                            position: coordinate.clone(),
                        });
                        circuit.add_variable(CircuitVariable {
                            name: def.name.clone(),
                            data: DigitalData::Wire(1, coordinate.clone()),
                            undefined: false,
                        });
                    }
                }
                Decorator::Out(bits, name) => {
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
                                                value: EntryValue::String(
                                                    name.clone()
                                                        .unwrap_or_else(|| def.name.clone()),
                                                ),
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
                                    undefined: false,
                                });
                            } else {
                                panic!("b: Output variable {} has no value", def.name);
                            }
                        } else {
                            panic!("c: Output variable {} has no value", def.name);
                        }
                    }
                }
                Decorator::Wire(width) => {
                    for def in self.definitions.iter() {
                        let coordinate = Coordinate::next();

                        circuit.add_variable(CircuitVariable {
                            name: def.name.clone(),
                            data: DigitalData::Wire(*width, coordinate.clone()),
                            undefined: true,
                        });
                    }
                }
            },
        }

        DigitalData::Empty
    }
}
