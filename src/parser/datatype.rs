use std::{collections::HashMap, sync::Arc};

use super::ParserState;

mod expression;

#[derive(Clone, Debug)]
pub enum KnownBitWidth {
    Fixed(u32),
    Object(HashMap<String, Arc<KnownBitWidth>>),
}

impl KnownBitWidth {
    pub fn get_size(&self) -> u32 {
        match self {
            KnownBitWidth::Fixed(size) => *size,
            KnownBitWidth::Object(map) => {
                if map.keys().len() != 1 {
                    panic!("Object width has more than one key");
                }

                map.values().next().unwrap().get_size()
            }
        }
    }
    pub fn max(left: KnownBitWidth, right: KnownBitWidth) -> KnownBitWidth {
        KnownBitWidth::Fixed(left.get_size().max(right.get_size()))
    }
}

pub trait GetBitWidth {
    fn get_bit_width(&self, state: &ParserState) -> KnownBitWidth;
}
