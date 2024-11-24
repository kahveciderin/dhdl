use winnow::{combinator, PResult, Parser};

use crate::{parser::ParserModuleVariable, types::module::Module};

use super::{
    identifier::parse_identifier,
    program::parse_program_statement,
    trivial_tokens::{parse_close_scope, parse_open_scope},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_module(input: &mut Stream) -> PResult<Module> {
    parse_whitespace(input)?;

    let name = parse_identifier.map(|s| s.to_string()).parse_next(input)?;

    parse_open_scope(input)?;

    input.state.start_new_module();

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
