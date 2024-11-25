use winnow::{combinator, PResult, Parser};

use crate::{
    digital::{Coordinate, Entry, EntryValue},
    parser::ParserModuleVariable,
    types::{
        expression::Expression,
        module::{ExternalModule, ExternalModuleVariableData, Module},
    },
};

use super::{
    argument::parse_arguments,
    datatype::KnownBitWidth,
    identifier::parse_identifier,
    number::parse_signed_number,
    program::parse_program_statement,
    trivial_tokens::{
        parse_at, parse_close_paren, parse_close_scope, parse_colon, parse_comma, parse_equals,
        parse_open_paren, parse_open_scope, parse_star,
    },
    whitespace::parse_whitespace,
    ParserModuleVariableData, Stream,
};

enum ExternalModuleVariableType {
    Input,
    Output,
}

fn parse_external_module_variable(
    input: &mut Stream,
) -> PResult<(ExternalModuleVariableData, ExternalModuleVariableType)> {
    parse_whitespace(input)?;

    let variable_type = combinator::preceded(parse_at, parse_identifier)
        .map(|s| match s {
            "in" => ExternalModuleVariableType::Input,
            "out" => ExternalModuleVariableType::Output,
            _ => unreachable!(),
        })
        .parse_next(input)?;

    let arguments = parse_arguments(input)?;

    let bit_count = if let Expression::Integer(bit_count) = arguments
        .get("bits")
        .or_else(|| arguments.get("0"))
        .unwrap_or_else(|| panic!("Missing bits argument"))
        .value
        .expression
    {
        KnownBitWidth::Fixed(bit_count)
    } else {
        panic!("Bits argument must be a constant")
    };

    let variable_name = parse_identifier.map(|s| s.to_string()).parse_next(input)?;

    let position = combinator::preceded(
        parse_at,
        combinator::delimited(
            parse_open_paren,
            combinator::separated_pair(parse_signed_number, parse_comma, parse_signed_number),
            parse_close_paren,
        ),
    )
    .map(|(x, y)| Coordinate {
        x: x.into(),
        y: y.into(),
    })
    .parse_next(input)?;

    Ok((
        ExternalModuleVariableData {
            name: variable_name,
            width: bit_count,
            position,
        },
        variable_type,
    ))
}

fn parse_entry_value(input: &mut Stream) -> PResult<EntryValue> {
    // todo: long distinction

    combinator::alt((
        parse_signed_number.map(|n| EntryValue::Integer(n)),
        parse_identifier.map(|s| EntryValue::String(s.to_string())),
    ))
    .parse_next(input)
}

fn parse_external_module_attribute(input: &mut Stream) -> PResult<Entry> {
    parse_whitespace(input)?;

    combinator::seq!(Entry {
        name: parse_identifier.map(|s| s.to_string()),
        value: combinator::preceded(parse_equals, parse_entry_value)
    })
    .parse_next(input)
}

enum ExternalModuleBodyItem {
    Variable(ExternalModuleVariableData, ExternalModuleVariableType),
    Attribute(Entry),
}

fn parse_external_module_body_item(input: &mut Stream) -> PResult<ExternalModuleBodyItem> {
    combinator::alt((
        parse_external_module_variable.map(|(data, ty)| ExternalModuleBodyItem::Variable(data, ty)),
        parse_external_module_attribute.map(ExternalModuleBodyItem::Attribute),
    ))
    .parse_next(input)
}

pub fn parse_external_module(input: &mut Stream) -> PResult<ExternalModule> {
    parse_whitespace(input)?;

    let name = combinator::preceded(parse_star, parse_identifier)
        .map(|s| s.to_string())
        .parse_next(input)?;

    let rename = combinator::opt(combinator::preceded(parse_colon, parse_identifier))
        .map(|s| s.map(|s| s.to_string()))
        .parse_next(input)?;

    parse_open_scope(input)?;

    input.state.start_new_module(name.clone());

    let body: Vec<_> =
        combinator::repeat_till(0.., parse_external_module_body_item, parse_close_scope)
            .map(|v| v.0)
            .parse_next(input)?;

    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut attributes = vec![];

    for item in body.iter() {
        match item {
            ExternalModuleBodyItem::Variable(data, ty) => match ty {
                ExternalModuleVariableType::Input => {
                    inputs.push(data.clone());
                    input.state.add_variable(ParserModuleVariable::Input(
                        ParserModuleVariableData {
                            name: data.name.clone(),
                            width: data.width.clone(),
                        },
                    ));
                }
                ExternalModuleVariableType::Output => {
                    outputs.push(data.clone());
                    input.state.add_variable(ParserModuleVariable::Output(
                        ParserModuleVariableData {
                            name: data.name.clone(),
                            width: data.width.clone(),
                        },
                    ));
                }
            },
            ExternalModuleBodyItem::Attribute(entry) => attributes.push(entry.clone()),
        }
    }

    input.state.end_current_module();

    Ok(ExternalModule {
        name,
        rename,
        inputs,
        outputs,
        attributes,
    })
}

pub fn parse_module(input: &mut Stream) -> PResult<Module> {
    parse_whitespace(input)?;

    let name = parse_identifier.map(|s| s.to_string()).parse_next(input)?;

    parse_open_scope(input)?;

    input.state.start_new_module(name.clone());

    let statements = combinator::repeat_till(0.., parse_program_statement, parse_close_scope)
        .map(|v| v.0)
        .parse_next(input)?;

    println!("Parsed module: {:?}", statements);

    let module = input.state.end_current_module();

    let mut inputs = vec![];
    let mut outputs = vec![];

    for variable in module.variables {
        match variable {
            ParserModuleVariable::Input(data) => inputs.push(data),
            ParserModuleVariable::Output(data) => outputs.push(data),
            ParserModuleVariable::Wire(_) => {}
        }
    }

    Ok(Module {
        name,
        statements,
        inputs,
        outputs,
    })
}
