use winnow::{combinator, token, PResult, Parser};

use super::{trivial_tokens::{parse_backslash, parse_quote}, whitespace::parse_whitespace, Stream};

pub fn parse_identifier<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;
    (
        token::one_of(('a'..='z', 'A'..='Z', '_')),
        token::take_while(0.., ('0'..='9', 'a'..='z', 'A'..='Z', '_')),
    )
        .take()
        .parse_next(input)
}

fn parse_string_character(input: &mut Stream) -> PResult<char> {
    combinator::alt((
        token::none_of(['\n', '\r', '"', '\\']),
        combinator::preceded(parse_backslash, token::one_of(['n', 'r', '\"', '\\'])).map(
            |c| match c {
                'n' => '\n',
                'r' => '\r',
                '"' => '"',
                '\\' => '\\',
                _ => unreachable!(),
            },
        ),
    ))
    .parse_next(input)
}

pub fn parse_string(input: &mut Stream) -> PResult<String> {
    parse_whitespace(input)?;

    combinator::preceded(
        parse_quote,
        combinator::repeat_till(0.., parse_string_character, parse_quote).map(|v| v.0),
    )
    .parse_next(input)
}
