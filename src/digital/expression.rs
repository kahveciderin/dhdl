use crate::{
    digital::{Entry, EntryValue, VisualElement, Wire},
    parser::datatype::KnownBitWidth,
    types::expression::{
        BinaryOp, Combine, Expression, ExpressionWithWidth, Extract, ExtractInner, UnaryOp,
    },
    utils::integer_width::integer_width,
};

use super::{Circuit, Coordinate, ToDigital};

impl ToDigital for ExpressionWithWidth {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate> {
        self.expression.convert_to_digital(circuit)
    }
}

impl ToDigital for Expression {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate> {
        match self {
            Expression::Integer(value) => {
                let coordinate = Coordinate::next();

                circuit.visual_elements.push(VisualElement {
                    name: String::from("Const"),
                    attributes: vec![
                        Entry {
                            name: String::from("Value"),
                            value: EntryValue::Long((*value).into()),
                        },
                        Entry {
                            name: String::from("Bits"),
                            value: EntryValue::Integer(integer_width(*value) as i32),
                        },
                    ],
                    position: coordinate.clone(),
                });

                vec![coordinate]
            }
            Expression::Variable(variable) => {
                let var = circuit.variables.iter().find(|v| v.name == *variable);

                match var {
                    Some(var) => vec![var.position.clone()],
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
    ($lhs:ident, $rhs:ident, $circuit:ident, $coordinate:ident, $name:literal) => {{
        let lhs_wire_positions = $lhs.convert_to_digital($circuit);
        let rhs_wire_positions = $rhs.convert_to_digital($circuit);

        $circuit.visual_elements.push(VisualElement {
            name: String::from($name),
            attributes: vec![Entry {
                name: String::from("wideShape"),
                value: EntryValue::Boolean(true),
            }],
            position: $coordinate.clone(),
        });

        $circuit.wires.push(Wire {
            start: lhs_wire_positions[0].clone(),
            end: $coordinate.clone(),
        });

        $circuit.wires.push(Wire {
            start: rhs_wire_positions[0].clone(),
            end: $coordinate.add(0, 40),
        });
    }};
}

macro_rules! match_binary_ops {
    ($op:ident, $circuit:ident, $coordinate:ident, [$($name:ident : $gate_name:literal : $end_x: literal),*]) => {
        match $op {
            $(BinaryOp::$name(lhs, rhs) => {
                binary_op_inner!(lhs, rhs, $circuit, $coordinate, $gate_name);
                vec![$coordinate.add($end_x, 20)]
            })*
        }
    };
}

impl ToDigital for BinaryOp {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate> {
        let coordinate = Coordinate::next();

        match_binary_ops!(
            self,
            circuit,
            coordinate,
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
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate> {
        let coordinate = Coordinate::next();

        match self {
            UnaryOp::Not(expression) => {
                let expression_wire_positions = expression.convert_to_digital(circuit);
                circuit.visual_elements.push(VisualElement {
                    name: String::from("Not"),
                    attributes: vec![],
                    position: coordinate.clone(),
                });

                circuit.wires.push(Wire {
                    start: expression_wire_positions[0].clone(),
                    end: coordinate.clone(),
                });
            }
        }

        vec![coordinate.add(40, 0)]
    }
}

impl ToDigital for Combine {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate> {
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
                        start: expr_coordinate[0].clone(),
                        end: value_coordinate.clone(),
                    });
                }

                vec![coordinate.add(20, 0)]
            }
            Combine::Obj(_) => todo!("Combine::Obj"),
        }
    }
}

impl ToDigital for Extract {
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate> {
        match self.extract {
            ExtractInner::Bit(bit) => {
                let coordinate = Coordinate::next();

                let input = self.expression.convert_to_digital(circuit);

                if let KnownBitWidth::Fixed(bit_width) = self.expression.width {
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
                        start: input[0].clone(),
                        end: coordinate.clone(),
                    });

                    vec![coordinate.add(20, 0)]
                } else {
                    panic!("Extracting a bit from an object variable")
                }
            }
            ExtractInner::Range(_, _) => todo!(),
            ExtractInner::Name(_) => todo!(),
        }
    }
}
