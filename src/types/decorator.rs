pub type BitWidth = u32;

#[derive(Debug, Clone)]
pub enum Decorator {
    Out(Option<BitWidth>, Option<String>),
    In(BitWidth, Option<String>),
    Wire(BitWidth),
    Clock(Option<BitWidth>),
}
