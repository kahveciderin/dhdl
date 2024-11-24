pub fn integer_width(int: u32) -> u32 {
    if int == 0 {
        1
    } else {
        int.ilog2() + 1
    }
}
