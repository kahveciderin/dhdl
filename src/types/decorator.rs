pub type BitWidth = u32;

#[derive(Debug, Clone)]
pub enum Decorator {
    Out(Option<BitWidth>),
    In(BitWidth),
}
