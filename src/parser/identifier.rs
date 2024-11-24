use winnow::{token, PResult, Parser};

use super::{whitespace::parse_whitespace, Stream};

pub fn parse_identifier<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;
    (
        token::one_of(('a'..='z', 'A'..='Z', '_')),
        token::take_while(0.., ('0'..='9', 'a'..='z', 'A'..='Z', '_')),
    )
        .take()
        .parse_next(input)
}