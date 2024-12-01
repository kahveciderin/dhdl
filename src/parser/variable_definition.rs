use winnow::{combinator, PResult, Parser};

use crate::{
    parser::{
        decorator::parse_decorator,
        expression::parse_expression,
        identifier::parse_identifier,
        trivial_tokens::{parse_comma, parse_equals},
    },
    types::{
        decorator::Decorator,
        expression::ExpressionWithWidth,
        variable_definition::{VariableDefinition, VariableDefinitions},
    },
};

use super::{
    datatype::{GetBitWidth, KnownBitWidth},
    whitespace::parse_whitespace,
    ParserModuleVariable, ParserModuleVariableData, Stream,
};

fn parse_variable_definition(input: &mut Stream) -> PResult<VariableDefinition> {
    parse_whitespace(input)?;

    let name = parse_identifier.map(|s| s.to_string()).parse_next(input)?;
    let value =
        combinator::opt(combinator::preceded(parse_equals, parse_expression)).parse_next(input)?;

    Ok(VariableDefinition {
        name,
        value: if let Some(value) = value {
            Some(ExpressionWithWidth {
                width: value.get_bit_width(&input.state),
                expression: value,
            })
        } else {
            None
        },
    })
}

pub fn parse_variable_definitions(input: &mut Stream) -> PResult<VariableDefinitions> {
    parse_whitespace(input)?;

    let definitions = combinator::seq!(VariableDefinitions {
        decorator: combinator::opt(parse_decorator),
        definitions: combinator::separated(0.., parse_variable_definition, parse_comma)
    })
    .parse_next(input)?;

    for definition in &definitions.definitions {
        if let Some(ref decorator) = definitions.decorator {
            let variable = match decorator {
                Decorator::In(width, name) => {
                    ParserModuleVariable::Input(ParserModuleVariableData {
                        name: definition.name.clone(),
                        external_name: name.clone().map_or(definition.name.clone(), |s| s),
                        width: KnownBitWidth::Fixed(*width),
                    })
                }
                Decorator::Out(width, name) => {
                    ParserModuleVariable::Output(ParserModuleVariableData {
                        name: definition.name.clone(),
                        external_name: name.clone().map_or(definition.name.clone(), |s| s),
                        width: if let Some(width) = width {
                            KnownBitWidth::Fixed(*width)
                        } else {
                            definition
                                .value
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("Output variable {} has no value", definition.name)
                                })
                                .width
                                .clone()
                        },
                    })
                }
                Decorator::Wire(width) => {
                    ParserModuleVariable::UndefinedWire(ParserModuleVariableData {
                        name: definition.name.clone(),
                        external_name: definition.name.clone(),
                        width: KnownBitWidth::Fixed(*width),
                    })
                }
            };
            input.state.add_variable(variable);
        } else {
            let expression = definition.value.as_ref();

            if let Some(expression) = expression {
                input.state.add_variable(ParserModuleVariable::DefinedWire(
                    ParserModuleVariableData {
                        name: definition.name.clone(),
                        external_name: definition.name.clone(),
                        width: expression.width.clone(),
                    },
                ));
            } else {
                return Err(winnow::error::ErrMode::Backtrack(
                    winnow::error::ContextError::new(),
                ));
            }
        }
    }

    Ok(definitions)
}
