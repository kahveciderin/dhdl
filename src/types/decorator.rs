pub type BitWidth = u32;

#[derive(Debug)]
pub enum Decorator {
    Out(BitWidth),
    In(BitWidth),
}
