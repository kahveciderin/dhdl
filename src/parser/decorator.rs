use winnow::PResult;

use crate::types::{
    decorator::Decorator,
    expression::{Expression, ExpressionWithWidth},
};

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
            let bits = arguments.get("bits").or_else(|| arguments.get("0"));

            if let Some(bits) = bits {
                let bits = bits.value.clone().expression;

                if let Expression::Integer(bits) = bits {
                    Ok(Decorator::Out(Some(bits)))
                } else {
                    Err(winnow::error::ErrMode::Backtrack(
                        winnow::error::ContextError::new(),
                    ))
                }
            } else {
                Ok(Decorator::Out(None))
            }
        }
        "in" => {
            let bits = arguments.get("bits").or_else(|| arguments.get("0"));

            if let Some(bits) = bits {
                let bits = bits.value.clone().expression;

                if let Expression::Integer(bits) = bits {
                    Ok(Decorator::In(bits))
                } else {
                    Err(winnow::error::ErrMode::Backtrack(
                        winnow::error::ContextError::new(),
                    ))
                }
            } else {
                Ok(Decorator::In(1))
            }
        }

        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        )),
    }
}
