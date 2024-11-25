use rand::Rng;
use std::iter;

pub fn random_name(prefix: Option<&str>, len: Option<usize>) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    let random_string = iter::repeat_with(one_char)
        .take(len.unwrap_or(32))
        .collect();
    match prefix {
        Some(prefix) => format!("{}{}", prefix, random_string),
        None => random_string,
    }
}

static mut IDENTIFIER_COUNT: u64 = 0;

pub fn unique_identifier(prefix: Option<&str>, len: Option<usize>) -> String {
    let prefix = match prefix {
        Some(prefix) => prefix.to_owned() + "____",
        None => "id____".to_owned(),
    };

    let random_name = "____".to_owned()
        + &random_name(Some(&prefix), len)
        + "____"
        + &unsafe { IDENTIFIER_COUNT.to_string() };

    unsafe {
        IDENTIFIER_COUNT += 1;
    };

    random_name
}
