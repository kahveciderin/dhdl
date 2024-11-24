use winnow::{token, PResult, Parser};

use super::Stream;

// todo: implement line breaking \
// and multi-line comments
fn parse_comment(input: &mut Stream) -> PResult<()> {
    "//".parse_next(input)?;

    token::take_while(0.., |c| c != '\n')
        .void()
        .parse_next(input)
}

fn parse_whitespace_inner(input: &mut Stream) -> PResult<()> {
    token::take_while(0.., (' ', '\n', '\t', '\r'))
        .void()
        .parse_next(input)
}

pub fn parse_whitespace(input: &mut Stream) -> PResult<()> {
    parse_whitespace_inner(input)?;

    loop {
        match parse_comment(input) {
            Ok(_) => parse_whitespace_inner(input)?,
            Err(_) => break Ok(()),
        }
    }
}
