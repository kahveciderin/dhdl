pub type BitWidth = u32;

#[derive(Debug)]
pub enum Decorator {
    Out(Option<BitWidth>),
    In(BitWidth),
}
