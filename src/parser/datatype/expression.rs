use crate::{
    parser::ParserState,
    types::expression::{
        BinaryOp, Combine, Expression, ExpressionWithWidth, Extract, ExtractInner, UnaryOp,
    },
    utils::integer_width::integer_width,
};

use super::{GetBitWidth, KnownBitWidth};

impl GetBitWidth for Expression {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            Expression::Integer(number) => KnownBitWidth::Fixed(integer_width(*number)),
            Expression::Variable(variable) => state
                .find_variable(&variable)
                .expect(&format!("Variable {} not found", variable))
                .get_bit_width(state),
            Expression::UnaryOp(op) => op.get_bit_width(state),
            Expression::BinaryOp(op) => op.get_bit_width(state),
            Expression::Extract(extract) => extract.get_bit_width(state),
            Expression::Combine(combine) => combine.get_bit_width(state),
            Expression::ModuleUse(_) => todo!(),
        }
    }
}

impl GetBitWidth for UnaryOp {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            UnaryOp::Not(expr) => expr.get_bit_width(state),
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
            | BinaryOp::XNOr(lhs, rhs) => {
                KnownBitWidth::max(lhs.get_bit_width(state), rhs.get_bit_width(state))
            }
        }
    }
}

impl GetBitWidth for Extract {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self.extract {
            ExtractInner::Bit(_) => KnownBitWidth::Fixed(1),
            ExtractInner::Range(start, end) => KnownBitWidth::Fixed((end - start) as u32),
            ExtractInner::Name(_) => todo!(),
        }
    }
}

impl GetBitWidth for Combine {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth {
        match self {
            Combine::Bits(bits) => KnownBitWidth::Fixed(bits.len() as u32),
            Combine::Obj(_) => todo!(),
        }
    }
}
