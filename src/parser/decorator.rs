use winnow::PResult;

use crate::types::{decorator::Decorator, expression::Expression};

use super::{
    argument::parse_arguments, identifier::parse_identifier, trivial_tokens::parse_at,
    whitespace::parse_whitespace, Stream,
};

pub fn parse_decorator(input: &mut Stream) -> PResult<Decorator> {
    parse_whitespace(input)?;

    parse_at(input)?;

    let decorator = parse_identifier(input)?;

    let arguments = parse_arguments(input)?;

    match decorator {
        "out" => {
            let bits = arguments
                .get("bits")
                .or_else(|| arguments.get("0"))
                .and_then(|arg| {
                    if let Expression::Integer(bits) = arg.value.clone().expression {
                        Some(bits)
                    } else {
                        None
                    }
                });

            let name = arguments.get("name").and_then(|arg| {
                if let Expression::String(name) = arg.value.clone().expression {
                    Some(name)
                } else {
                    None
                }
            });

            Ok(Decorator::Out(bits, name))
        }
        "in" => {
            let bits = arguments
                .get("bits")
                .or_else(|| arguments.get("0"))
                .and_then(|arg| {
                    if let Expression::Integer(bits) = arg.value.clone().expression {
                        Some(bits)
                    } else {
                        None
                    }
                })
                .map_or(1, |x| x);

            let name = arguments.get("name").and_then(|arg| {
                if let Expression::String(name) = arg.value.clone().expression {
                    Some(name)
                } else {
                    None
                }
            });

            Ok(Decorator::In(bits, name))
        }
        "wire" => {
            let bits = arguments
                .get("bits")
                .or_else(|| arguments.get("0"))
                .and_then(|arg| {
                    if let Expression::Integer(bits) = arg.value.clone().expression {
                        Some(bits)
                    } else {
                        None
                    }
                })
                .map_or(1, |x| x);

            Ok(Decorator::Wire(bits))
        }

        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        )),
    }
}
