use std::sync::Arc;

use crate::{
    parser::ParserState,
    types::expression::{BinaryOp, Combine, Expression, Extract, ExtractInner, ModuleUse, UnaryOp},
    utils::integer_width::integer_width,
};

use super::{GetBitWidth, KnownBitWidth};

impl GetBitWidth for Expression {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            Expression::Integer(number) => KnownBitWidth::Fixed(integer_width(*number)),
            Expression::Variable(variable) => state
                .find_variable(variable)
                .unwrap_or_else(|| panic!("! Variable {} not found", variable))
                .get_bit_width(state),
            Expression::UnaryOp(op) => op.get_bit_width(state),
            Expression::BinaryOp(op) => op.get_bit_width(state),
            Expression::Extract(extract) => extract.get_bit_width(state),
            Expression::Combine(combine) => combine.get_bit_width(state),
            Expression::ModuleUse(module_use) => module_use.get_bit_width(state),
        }
    }
}

impl GetBitWidth for UnaryOp {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            UnaryOp::Not(expr) => expr.width.clone(),
        }
    }
}

impl GetBitWidth for BinaryOp {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            BinaryOp::And(lhs, rhs)
            | BinaryOp::NAnd(lhs, rhs)
            | BinaryOp::Or(lhs, rhs)
            | BinaryOp::NOr(lhs, rhs)
            | BinaryOp::XOr(lhs, rhs)
            | BinaryOp::XNOr(lhs, rhs) => KnownBitWidth::max(lhs.width.clone(), rhs.width.clone()),
        }
    }
}

impl GetBitWidth for Extract {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match &self.extract {
            ExtractInner::Bit(_) => KnownBitWidth::Fixed(1),
            ExtractInner::Range(start, end) => {
                if start > end {
                    panic!("Start index must be less than or equal to end index");
                }

                KnownBitWidth::Fixed(1 + (end - start))
            }
            ExtractInner::Name(key) => {
                let self_bit_width = &self.expression.width;

                if let KnownBitWidth::Object(map) = self_bit_width {
                    map.get(key.as_str())
                        .unwrap_or_else(|| panic!("Key {} not found in object", key))
                        .as_ref()
                        .clone()
                } else {
                    panic!("Extracting from non-object");
                }
            }
        }
    }
}

impl GetBitWidth for Combine {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            Combine::Bits(bits) => KnownBitWidth::Fixed(bits.len() as u32),
            Combine::Obj(values) => KnownBitWidth::Object(
                values
                    .iter()
                    .map(|(key, value)| (key.clone(), Arc::new(value.get_bit_width(state))))
                    .collect(),
            ),
        }
    }
}

impl GetBitWidth for ModuleUse {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        let module = state
            .find_module(&self.name)
            .unwrap_or_else(|| panic!("Module {} not found", self.name));

        let map = module
            .outputs
            .iter()
            .map(|output| (output.name.clone(), Arc::new(output.width.clone())))
            .collect();

        KnownBitWidth::Object(map)
    }
}
