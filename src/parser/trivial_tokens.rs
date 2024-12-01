use winnow::{PResult, Parser};

use super::{whitespace::parse_whitespace, Stream};

pub fn parse_multiple_chars<'s>(input: &mut Stream<'s>, mut s: &str) -> PResult<&'s str> {
    parse_whitespace(input)?;
    s.parse_next(input)
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

pub fn parse_star<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "*")
}

pub fn parse_minus<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "-")
}

pub fn parse_quote<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "\"")
}

pub fn parse_backslash<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "\\")
}

pub fn parse_percent<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "%")
}

pub fn parse_true<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "true")
}
pub fn parse_false<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "false")
}

pub fn parse_rgb<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "rgb")
}
pub fn parse_rgba<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "rgba")
}

pub fn parse_up<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "up")
}
pub fn parse_down<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "down")
}
pub fn parse_left<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "left")
}
pub fn parse_right<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "right")
}
