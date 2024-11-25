use crate::{
    digital::{CircuitVariable, Coordinate, Entry, EntryValue, VisualElement, Wire},
    parser::datatype::KnownBitWidth,
    types::{decorator::Decorator, variable_definition::VariableDefinitions},
};

use super::{Circuit, ToDigital};

pub fn cast_value(
    from: KnownBitWidth,
    to: KnownBitWidth,
    value: Coordinate,
    circuit: &mut Circuit,
) -> Coordinate {
    match (from, to) {
        (KnownBitWidth::Fixed(from), KnownBitWidth::Fixed(to)) => {
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
                    start: value.clone(),
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
                    start: value.clone(),
                    end: coordinate.clone(),
                });

                coordinate.add(20, 0)
            } else {
                value
            }
        }
    }
}

impl ToDigital for VariableDefinitions {
    fn convert_to_digital(&self, circuit: &mut super::Circuit) -> Vec<super::Coordinate> {
        match &self.decorator {
            None => {
                for def in self.definitions.iter() {
                    if let Some(expression) = &def.value {
                        let coordinate = expression.convert_to_digital(circuit);

                        circuit.variables.push(CircuitVariable {
                            name: def.name.clone(),
                            position: coordinate[0].clone(),

                            width: expression.width.clone(),
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
                        circuit.variables.push(CircuitVariable {
                            name: def.name.clone(),
                            position: coordinate.clone(),

                            width: KnownBitWidth::Fixed(*bits),
                        });
                    }
                }
                Decorator::Out(bits) => {
                    for def in self.definitions.iter() {
                        let coordinate = Coordinate::next();
                        circuit.visual_elements.push(VisualElement {
                            name: String::from("Out"),
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

                        if let Some(expression) = &def.value {
                            let input_wire_position = expression.convert_to_digital(circuit);
                            let casted_value = cast_value(
                                expression.width.clone(),
                                KnownBitWidth::Fixed(*bits),
                                input_wire_position[0].clone(),
                                circuit,
                            );

                            circuit.wires.push(Wire {
                                start: casted_value,
                                end: coordinate.clone(),
                            })
                        } else {
                            panic!("Output variable {} has no value", def.name);
                        }
                    }
                }
            },
        }

        vec![]
    }
}
