use winnow::{combinator, token, PResult, Parser};

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
        parse_L, parse_at, parse_backslash, parse_close_paren, parse_close_scope, parse_colon,
        parse_comma, parse_equals, parse_false, parse_l, parse_open_paren, parse_open_scope,
        parse_quote, parse_rgb, parse_rgba, parse_star, parse_true,
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
        x,
        y,
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

fn parse_string(input: &mut Stream) -> PResult<String> {
    parse_whitespace(input)?;

    combinator::preceded(
        parse_quote,
        combinator::repeat_till(0.., parse_string_character, parse_quote).map(|v| v.0),
    )
    .parse_next(input)
}

fn parse_rgba_color(input: &mut Stream) -> PResult<(u8, u8, u8, u8)> {
    parse_whitespace(input)?;

    combinator::preceded(
        parse_rgba,
        combinator::delimited(
            parse_open_paren,
            combinator::separated(4, parse_signed_number, parse_comma)
                .map(|v: Vec<_>| (v[0] as u8, v[1] as u8, v[2] as u8, v[3] as u8)),
            parse_close_paren,
        ),
    )
    .parse_next(input)
}
fn parse_rgb_color(input: &mut Stream) -> PResult<(u8, u8, u8, u8)> {
    parse_whitespace(input)?;

    combinator::preceded(
        parse_rgb,
        combinator::delimited(
            parse_open_paren,
            combinator::separated(3, parse_signed_number, parse_comma)
                .map(|v: Vec<_>| (v[0] as u8, v[1] as u8, v[2] as u8, 255)),
            parse_close_paren,
        ),
    )
    .parse_next(input)
}

fn parse_long(input: &mut Stream) -> PResult<i64> {
    parse_whitespace(input)?;

    combinator::terminated(parse_signed_number, combinator::alt((parse_l, parse_L)))
        .parse_next(input)
}

fn parse_entry_value(input: &mut Stream) -> PResult<EntryValue> {
    combinator::alt((
        parse_long.map(EntryValue::Long),
        parse_signed_number.map(|v| EntryValue::Integer(v as i32)),
        parse_string.map(|s| EntryValue::String(s.to_string())),
        parse_true.map(|_| EntryValue::Boolean(true)),
        parse_false.map(|_| EntryValue::Boolean(true)),
        parse_rgb_color.map(|(r, g, b, a)| EntryValue::Color((r, g, b, a))),
        parse_rgba_color.map(|(r, g, b, a)| EntryValue::Color((r, g, b, a))),
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

    let mut name = combinator::preceded(parse_star, parse_identifier)
        .map(|s| s.to_string())
        .parse_next(input)?;

    let rename = combinator::opt(combinator::preceded(parse_colon, parse_identifier))
        .map(|s| s.map(|s| s.to_string()))
        .parse_next(input)?;

    let rename_str;
    if let Some(original_name) = rename {
        let rename = name.clone();
        name = original_name;
        rename_str = rename;
    } else {
        rename_str = name.clone();
    }

    parse_open_scope(input)?;

    input.state.start_new_module(rename_str.clone());

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
        internal_name: name,
        name: rename_str,
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
