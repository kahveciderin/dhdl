use std::collections::HashMap;

use winnow::{combinator, PResult, Parser};

use crate::types::{argument::Argument, expression::ExpressionWithWidth};

use super::{
    datatype::GetBitWidth,
    expression::parse_expression,
    identifier::parse_identifier,
    trivial_tokens::{parse_close_paren, parse_colon, parse_comma, parse_open_paren},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_argument(input: &mut Stream) -> PResult<Argument> {
    parse_whitespace(input)?;

    let name = combinator::opt(
        combinator::terminated(parse_identifier, parse_colon).map(|s| s.to_string()),
    )
    .parse_next(input)?;

    let value = parse_expression(input)?;

    Ok(Argument {
        name,
        value: ExpressionWithWidth {
            width: value.get_bit_width(&input.state),
            expression: value,
        },
    })
}

pub fn parse_arguments_inner(input: &mut Stream) -> PResult<HashMap<String, Argument>> {
    parse_whitespace(input)?;
    let arguments = combinator::separated(0.., parse_argument, parse_comma).parse_next(input)?;
    Ok(create_argument_map(arguments))
}

pub fn parse_arguments(input: &mut Stream) -> PResult<HashMap<String, Argument>> {
    parse_whitespace(input)?;

    let open_paren = parse_open_paren(input);

    if open_paren.is_err() {
        Ok(create_argument_map(vec![]))
    } else {
        combinator::terminated(parse_arguments_inner, parse_close_paren).parse_next(input)
    }
}

pub fn create_argument_map(arguments: Vec<Argument>) -> HashMap<String, Argument> {
    let mut map = HashMap::new();

    for (i, arg) in arguments.into_iter().enumerate() {
        map.insert(arg.name.clone().unwrap_or(i.to_string()), arg);
    }
    map
}
