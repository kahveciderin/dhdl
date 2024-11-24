use super::ParserState;

mod expression;

#[derive(Clone, Debug)]
pub enum KnownBitWidth {
    Fixed(u32),
}

impl KnownBitWidth {
    pub fn max(left: KnownBitWidth, right: KnownBitWidth) -> KnownBitWidth {
        match (left, right) {
            (KnownBitWidth::Fixed(left), KnownBitWidth::Fixed(right)) => {
                KnownBitWidth::Fixed(left.max(right))
            }
        }
    }
}

pub trait GetBitWidth {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth;
}
