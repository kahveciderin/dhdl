use std::{collections::HashMap, sync::Arc};

use crate::{
    digital::{variable_definition::cast_value, Entry, EntryValue, VisualElement, Wire},
    parser::datatype::KnownBitWidth,
    types::expression::{
        BinaryOp, Combine, Expression, ExpressionWithWidth, Extract, ExtractInner, UnaryOp,
    },
    utils::integer_width::integer_width,
};

use super::{Circuit, Coordinate, DigitalData, ToDigital};

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
                let var = circuit.variables.iter().find(|v| v.name == *variable);

                match var {
                    Some(var) => var.data.clone(),
                    None => panic!("Variable {} not found", variable),
                }
            }
            Expression::UnaryOp(op) => op.convert_to_digital(circuit),
            Expression::BinaryOp(op) => op.convert_to_digital(circuit),
            Expression::Extract(extract) => extract.convert_to_digital(circuit),
            Expression::Combine(combine) => combine.convert_to_digital(circuit),
            Expression::ModuleUse(_) => todo!("ModuleUse"),
        }
    }
}

macro_rules! binary_op_inner {
    ($lhs:ident, $rhs:ident, $circuit:ident,  $name:literal, $end_x: literal) => {{
        let lhs_wire_positions = $lhs.convert_to_digital($circuit);
        let rhs_wire_positions = $rhs.convert_to_digital($circuit);

        let largest_type = KnownBitWidth::max($lhs.width.clone(), $rhs.width.clone());

        let lhs_casted = cast_value(
            $lhs.width.clone(),
            largest_type.clone(),
            lhs_wire_positions,
            $circuit,
        );

        let rhs_casted = cast_value(
            $rhs.width.clone(),
            largest_type.clone(),
            rhs_wire_positions,
            $circuit,
        );

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

                let ret = DigitalData::Object(obj);
                println!("combine: {:?}", ret);
                ret
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
                let input_casted = cast_value(
                    self.expression.width.clone(),
                    KnownBitWidth::Fixed(to + 1),
                    input,
                    circuit,
                );

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

                println!("extraction from: {:?}", input);

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
