use winnow::{combinator, token, PResult, Parser};

use super::{trivial_tokens::parse_minus, whitespace::parse_whitespace, Stream};

pub fn parse_hex_number(input: &mut Stream) -> PResult<u64> {
    parse_whitespace(input)?;

    "0x".parse_next(input)?;

    token::take_while(2.., |c: char| c.is_ascii_hexdigit() || c == '_')
        .parse_next(input)
        .map(|s| u64::from_str_radix(&s.replace("_", ""), 16).unwrap())
}

pub fn parse_binary_number(input: &mut Stream) -> PResult<u64> {
    parse_whitespace(input)?;

    "0b".parse_next(input)?;

    token::take_while(2.., |c: char| c == '0' || c == '1' || c == '_')
        .parse_next(input)
        .map(|s| u64::from_str_radix(&s.replace("_", ""), 2).unwrap())
}

pub fn parse_octal_number(input: &mut Stream) -> PResult<u64> {
    parse_whitespace(input)?;

    "0o".parse_next(input)?;

    token::take_while(2.., |c: char| (c == '_' || c.is_ascii_digit()) && c != '8' && c != '9')
        .parse_next(input)
        .map(|s| u64::from_str_radix(&s.replace("_", ""), 8).unwrap())
}

pub fn parse_decimal_number(input: &mut Stream) -> PResult<u64> {
    parse_whitespace(input)?;

    token::take_while(1.., |c: char| c.is_ascii_digit() || c == '_')
        .parse_next(input)
        .map(|s| u64::from_str_radix(&s.replace("_", ""), 10).unwrap())
}

pub fn parse_number(input: &mut Stream) -> PResult<u64> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_hex_number,
        parse_binary_number,
        parse_octal_number,
        parse_decimal_number,
    ))
    .parse_next(input)
}

pub fn parse_number_u32(input: &mut Stream) -> PResult<u32> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_hex_number,
        parse_binary_number,
        parse_octal_number,
        parse_decimal_number,
    ))
    .map(|n| n as u32)
    .parse_next(input)
}

pub fn parse_signed_number(input: &mut Stream) -> PResult<i64> {
    parse_whitespace(input)?;

    let sign = combinator::opt(parse_minus).parse_next(input)?;

    let number = parse_number.parse_next(input)?;

    Ok(match sign {
        Some("-") => -(number as i64),
        _ => number as i64,
    })
}
