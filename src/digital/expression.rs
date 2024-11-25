use std::{collections::HashMap, sync::Arc};

use crate::{
    digital::{variable_definition::cast_value, Entry, EntryValue, VisualElement, Wire},
    parser::datatype::KnownBitWidth,
    types::expression::{
        BinaryOp, Combine, Expression, ExpressionWithWidth, Extract, ExtractInner, ModuleUse,
        UnaryOp,
    },
    utils::integer_width::integer_width,
};

use super::{
    Circuit, CircuitModule, CircuitVariable, Coordinate, CurrentModule, DigitalData, ToDigital,
};

impl ToDigital for ExpressionWithWidth {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        self.expression.convert_to_digital(circuit)
    }
}

impl ToDigital for Expression {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        match self {
            Expression::Integer(value) => {
                let coordinate = Coordinate::next();
                let width = integer_width(*value);

                circuit.visual_elements.push(VisualElement {
                    name: String::from("Const"),
                    attributes: vec![
                        Entry {
                            name: String::from("Value"),
                            value: EntryValue::Long((*value).into()),
                        },
                        Entry {
                            name: String::from("Bits"),
                            value: EntryValue::Integer(width as i32),
                        },
                    ],
                    position: coordinate.clone(),
                });

                DigitalData::Wire(width, coordinate)
            }
            Expression::Variable(variable) => {
                let var = circuit.find_variable((*variable).clone());

                match var {
                    Some(var) => var.data.clone(),
                    None => panic!("Variable {} not found", variable),
                }
            }
            Expression::UnaryOp(op) => op.convert_to_digital(circuit),
            Expression::BinaryOp(op) => op.convert_to_digital(circuit),
            Expression::Extract(extract) => extract.convert_to_digital(circuit),
            Expression::Combine(combine) => combine.convert_to_digital(circuit),
            Expression::ModuleUse(module_use) => module_use.convert_to_digital(circuit),
        }
    }
}

macro_rules! binary_op_inner {
    ($lhs:ident, $rhs:ident, $circuit:ident,  $name:literal, $end_x: literal) => {{
        let lhs_wire_positions = $lhs.convert_to_digital($circuit);
        let rhs_wire_positions = $rhs.convert_to_digital($circuit);

        let largest_type = KnownBitWidth::max($lhs.width.clone(), $rhs.width.clone());

        let lhs_casted = cast_value(lhs_wire_positions, largest_type.clone(), $circuit);

        let rhs_casted = cast_value(rhs_wire_positions, largest_type.clone(), $circuit);

        if let KnownBitWidth::Fixed(bit_width) = largest_type {
            let output_coordinate = Coordinate::next();

            $circuit.visual_elements.push(VisualElement {
                name: String::from("Splitter"),
                attributes: vec![
                    Entry {
                        name: String::from("Input Splitting"),
                        value: EntryValue::String(String::from("1 * ") + &bit_width.to_string()),
                    },
                    Entry {
                        name: String::from("Output Splitting"),
                        value: EntryValue::String(bit_width.to_string()),
                    },
                ],
                position: output_coordinate.clone(),
            });

            let lhs_splitter_coordinate = Coordinate::next();

            $circuit.visual_elements.push(VisualElement {
                name: String::from("Splitter"),
                attributes: vec![
                    Entry {
                        name: String::from("Input Splitting"),
                        value: EntryValue::String(bit_width.to_string()),
                    },
                    Entry {
                        name: String::from("Output Splitting"),
                        value: EntryValue::String(String::from("1 * ") + &bit_width.to_string()),
                    },
                ],
                position: lhs_splitter_coordinate.clone(),
            });

            let rhs_splitter_coordinate = Coordinate::next();

            $circuit.visual_elements.push(VisualElement {
                name: String::from("Splitter"),
                attributes: vec![
                    Entry {
                        name: String::from("Input Splitting"),
                        value: EntryValue::String(bit_width.to_string()),
                    },
                    Entry {
                        name: String::from("Output Splitting"),
                        value: EntryValue::String(String::from("1 * ") + &bit_width.to_string()),
                    },
                ],
                position: rhs_splitter_coordinate.clone(),
            });

            $circuit.wires.push(Wire {
                start: lhs_casted.clone(),
                end: lhs_splitter_coordinate.clone(),
            });

            $circuit.wires.push(Wire {
                start: rhs_casted.clone(),
                end: rhs_splitter_coordinate.clone(),
            });

            for i in 0..bit_width {
                let coordinate = Coordinate::next();

                $circuit.visual_elements.push(VisualElement {
                    name: String::from($name),
                    attributes: vec![Entry {
                        name: String::from("wideShape"),
                        value: EntryValue::Boolean(true),
                    }],
                    position: coordinate.clone(),
                });

                $circuit.wires.push(Wire {
                    start: lhs_splitter_coordinate.add(20, (20 * i).into()).clone(),
                    end: coordinate.clone(),
                });

                $circuit.wires.push(Wire {
                    start: rhs_splitter_coordinate.add(20, (20 * i).into()).clone(),
                    end: coordinate.add(0, 40),
                });

                $circuit.wires.push(Wire {
                    start: output_coordinate.add(0, (20 * i).into()).clone(),
                    end: coordinate.add($end_x, 20),
                });
            }

            DigitalData::Wire(bit_width, output_coordinate.add(20, 0))
        } else {
            panic!("Trying to perform binary operation on object variables");
        }
    }};
}

macro_rules! match_binary_ops {
    ($op:ident, $circuit:ident, [$($name:ident : $gate_name:literal : $end_x: literal),*]) => {
        match $op {
            $(BinaryOp::$name(lhs, rhs) => {
                binary_op_inner!(lhs, rhs, $circuit, $gate_name, $end_x)
            })*
        }
    };
}

impl ToDigital for BinaryOp {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        match_binary_ops!(
            self,
            circuit,
            [
                And: "And" : 80,
                NAnd: "NAnd" : 100,
                Or: "Or" : 80,
                NOr: "NOr" : 100,
                XOr: "XOr" : 80,
                XNOr: "XNOr" : 100
            ]
        )
    }
}

impl ToDigital for UnaryOp {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        match self {
            UnaryOp::Not(expression) => {
                if let KnownBitWidth::Fixed(bit_width) = expression.width {
                    let output_coordinate = Coordinate::next();

                    circuit.visual_elements.push(VisualElement {
                        name: String::from("Splitter"),
                        attributes: vec![
                            Entry {
                                name: String::from("Input Splitting"),
                                value: EntryValue::String(
                                    String::from("1 * ") + &bit_width.to_string(),
                                ),
                            },
                            Entry {
                                name: String::from("Output Splitting"),
                                value: EntryValue::String(bit_width.to_string()),
                            },
                        ],
                        position: output_coordinate.clone(),
                    });

                    let splitter_coordinate = Coordinate::next();

                    circuit.visual_elements.push(VisualElement {
                        name: String::from("Splitter"),
                        attributes: vec![
                            Entry {
                                name: String::from("Input Splitting"),
                                value: EntryValue::String(bit_width.to_string()),
                            },
                            Entry {
                                name: String::from("Output Splitting"),
                                value: EntryValue::String(
                                    String::from("1 * ") + &bit_width.to_string(),
                                ),
                            },
                        ],
                        position: splitter_coordinate.clone(),
                    });

                    let expression_wire_positions = expression.convert_to_digital(circuit);

                    circuit.wires.push(Wire {
                        start: expression_wire_positions.get_position().clone(),
                        end: splitter_coordinate.clone(),
                    });

                    for i in 0..bit_width {
                        let coordinate = Coordinate::next();

                        circuit.visual_elements.push(VisualElement {
                            name: String::from("Not"),
                            attributes: vec![],
                            position: coordinate.clone(),
                        });

                        circuit.wires.push(Wire {
                            start: splitter_coordinate.add(20, (20 * i).into()),
                            end: coordinate.clone(),
                        });

                        circuit.wires.push(Wire {
                            start: coordinate.add(40, 0),
                            end: output_coordinate.add(0, (20 * i).into()),
                        });
                    }

                    DigitalData::Wire(bit_width, output_coordinate.add(20, 0))
                } else {
                    panic!("Trying to perform unary operation on object variables");
                }
            }
        }
    }
}

impl ToDigital for Combine {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        match self {
            Combine::Bits(values) => {
                let coordinate = Coordinate::next();

                circuit.visual_elements.push(VisualElement {
                    name: String::from("Splitter"),
                    attributes: vec![
                        Entry {
                            name: String::from("Input Splitting"),
                            value: EntryValue::String(
                                String::from("1 * ") + &values.len().to_string(),
                            ),
                        },
                        Entry {
                            name: String::from("Output Splitting"),
                            value: EntryValue::String(values.len().to_string()),
                        },
                    ],
                    position: coordinate.clone(),
                });

                for (i, value) in values.iter().enumerate() {
                    let expr_coordinate = value.convert_to_digital(circuit);
                    let value_coordinate = coordinate.add(0, 20 * i as i64);

                    circuit.wires.push(Wire {
                        start: expr_coordinate.get_position().clone(),
                        end: value_coordinate.clone(),
                    });
                }

                DigitalData::Wire(values.len().try_into().unwrap(), coordinate.add(20, 0))
            }
            Combine::Obj(map) => {
                let mut obj = HashMap::new();

                for (key, value) in map {
                    obj.insert(key.clone(), Arc::new(value.convert_to_digital(circuit)));
                }

                
                DigitalData::Object(obj)
            }
        }
    }
}

impl ToDigital for Extract {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        match &self.extract {
            ExtractInner::Bit(bit) => {
                let coordinate = Coordinate::next();

                let input = self.expression.convert_to_digital(circuit);

                if let KnownBitWidth::Fixed(bit_width) = self.expression.width {
                    if *bit >= bit_width {
                        circuit.visual_elements.push(VisualElement {
                            name: String::from("Const"),
                            attributes: vec![
                                Entry {
                                    name: String::from("Value"),
                                    value: EntryValue::Long(0),
                                },
                                Entry {
                                    name: String::from("Bits"),
                                    value: EntryValue::Integer(1),
                                },
                            ],
                            position: coordinate.clone(),
                        });

                        DigitalData::Wire(1, coordinate.clone())
                    } else {
                        circuit.visual_elements.push(VisualElement {
                            name: String::from("Splitter"),
                            attributes: vec![
                                Entry {
                                    name: String::from("Input Splitting"),
                                    value: EntryValue::String(bit_width.to_string()),
                                },
                                Entry {
                                    name: String::from("Output Splitting"),
                                    value: EntryValue::String(
                                        bit.to_string() + " - " + &bit.to_string(),
                                    ),
                                },
                            ],
                            position: coordinate.clone(),
                        });

                        circuit.wires.push(Wire {
                            start: input.get_position().clone(),
                            end: coordinate.clone(),
                        });

                        DigitalData::Wire(1, coordinate.add(20, 0))
                    }
                } else {
                    panic!("Extracting a bit from an object variable")
                }
            }
            ExtractInner::Range(from, to) => {
                let coordinate = Coordinate::next();

                let input = self.expression.convert_to_digital(circuit);
                let input_casted = cast_value(input, KnownBitWidth::Fixed(to + 1), circuit);

                circuit.visual_elements.push(VisualElement {
                    name: String::from("Splitter"),
                    attributes: vec![
                        Entry {
                            name: String::from("Input Splitting"),
                            value: EntryValue::String((to + 1).to_string()),
                        },
                        Entry {
                            name: String::from("Output Splitting"),
                            value: EntryValue::String(from.to_string() + " - " + &to.to_string()),
                        },
                    ],
                    position: coordinate.clone(),
                });

                circuit.wires.push(Wire {
                    start: input_casted,
                    end: coordinate.clone(),
                });

                DigitalData::Wire(1 + (to - from), coordinate.add(20, 0))
            }
            ExtractInner::Name(name) => {
                let input = self.expression.convert_to_digital(circuit);

                if let DigitalData::Object(obj) = input {
                    if let Some(value) = obj.get(name) {
                        value.as_ref().clone()
                    } else {
                        panic!("Object does not have key {}", name);
                    }
                } else {
                    panic!("Extracting a key from a non-object variable");
                }
            }
        }
    }
}

impl ToDigital for ModuleUse {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData {
        let module = circuit.find_module(&self.name).cloned();

        if let Some(module) = module {
            match module {
                CircuitModule::Internal(module) => {
                    circuit
                        .current_module
                        .push(CurrentModule { variables: vec![] });

                    for input in module.inputs {
                        let argument = self
                            .arguments
                            .get(&input.name)
                            .or_else(|| {
                                if self.arguments.len() == 1 {
                                    self.arguments.get("0")
                                } else {
                                    None
                                }
                            })
                            .unwrap_or_else(|| {
                                panic!("Input {} not found", input.name);
                            });
                        let input_data = argument.value.convert_to_digital(circuit);
                        let input_data =
                            cast_value(input_data, argument.value.width.clone(), circuit);

                        if let KnownBitWidth::Fixed(width) = argument.value.width {
                            circuit.add_variable(CircuitVariable {
                                name: input.name.clone(),
                                data: DigitalData::Wire(width, input_data),
                            });
                        } else {
                            panic!("Width of input is not fixed")
                        }
                    }

                    for statement in &module.statements {
                        statement.convert_to_digital(circuit);
                    }

                    let output_variables = module.outputs.iter().map(|v| v.name.clone());

                    let mut map = HashMap::new();

                    for var in output_variables {
                        let var = circuit
                            .current_module
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .find(|v| v.name == var); // not using find_variable here for a reason
                        if let Some(var) = var {
                            map.insert(var.name.clone(), Arc::new(var.data.clone()));
                        } else {
                            panic!("Output variable not found");
                        }
                    }

                    circuit.current_module.pop();

                    DigitalData::Object(map)
                }

                CircuitModule::External(module) => {
                    let coordinate = Coordinate::next();

                    circuit.visual_elements.push(VisualElement {
                        name: module.internal_name.clone(),
                        attributes: module.attributes.clone(),
                        position: coordinate.clone(),
                    });

                    for (key, value) in self.arguments.iter() {
                        let wire_positions = value.value.convert_to_digital(circuit);

                        if let DigitalData::Wire(_, position) = wire_positions {
                            let additional_coordinate =
                                module.inputs.iter().find(|v| v.name == *key);

                            if let Some(additional_coordinate) = additional_coordinate {
                                circuit.wires.push(Wire {
                                    start: position,
                                    end: coordinate.add(
                                        additional_coordinate.position.x,
                                        additional_coordinate.position.y,
                                    ),
                                });
                            } else {
                                panic!("Module argument {} not found", key);
                            }
                        } else {
                            panic!("Module argument is not a wire");
                        }
                    }

                    let mut map = HashMap::new();

                    for output in module.outputs.iter() {
                        let output_coordinate =
                            coordinate.add(output.position.x, output.position.y);

                        if let KnownBitWidth::Fixed(width) = output.width {
                            let output_data = DigitalData::Wire(width, output_coordinate);

                            map.insert(output.name.clone(), Arc::new(output_data));
                        }
                    }

                    DigitalData::Object(map)
                }
            }
        } else {
            panic!("Module {} not found", self.name);
        }
    }
}
