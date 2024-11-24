use winnow::{combinator, PResult, Parser};

use super::{whitespace::parse_whitespace, Stream};

pub fn parse_multiple_chars<'s>(input: &mut Stream<'s>, mut s: &str) -> PResult<&'s str> {
    parse_whitespace(input)?;
    s.parse_next(input)
}

pub fn parse_multiple_chars_not_followed_by<'s>(
    input: &mut Stream<'s>,
    s: &str,
    n: &[&str],
) -> PResult<&'s str> {
    parse_whitespace(input)?;

    for not in n.iter() {
        combinator::not(combinator::terminated(s, *not)).parse_next(input)?;
    }

    parse_multiple_chars(input, s)
}

pub fn parse_at<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "@")
}

pub fn parse_open_paren<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "(")
}
pub fn parse_close_paren<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ")")
}

pub fn parse_colon<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ":")
}

pub fn parse_comma<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ",")
}

pub fn parse_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "=")
}

pub fn parse_amperstand<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "&")
}

pub fn parse_pipe<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "|")
}

pub fn parse_caret<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "^")
}

pub fn parse_bang_amperstand<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "!&")
}

pub fn parse_bang_pipe<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "!|")
}

pub fn parse_bang_caret<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "!^")
}

pub fn parse_bang<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "!")
}

pub fn parse_dot<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ".")
}

pub fn parse_double_dot<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "..")
}

pub fn parse_open_square_bracket<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "[")
}
pub fn parse_close_square_bracket<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "]")
}

pub fn parse_open_scope<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "{")
}
pub fn parse_close_scope<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "}")
}
