use winnow::{combinator, PResult, Parser};

use crate::{
    parser::expression::parse_expression,
    types::program::{Program, ProgramStatement},
};

use super::{
    module::{parse_external_module, parse_module},
    variable_definition::parse_variable_definitions,
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_program_statement(input: &mut Stream) -> PResult<ProgramStatement> {
    combinator::alt((
        parse_external_module.map(ProgramStatement::ExternalModule),
        parse_module.map(ProgramStatement::Module),
        parse_variable_definitions.map(ProgramStatement::VariableDefinitions),
        parse_expression.map(ProgramStatement::Expression),
    ))
    .parse_next(input)
}

pub fn parse_program(input: &mut Stream) -> PResult<Program> {
    parse_whitespace(input)?;

    let statements = combinator::repeat_till(0.., parse_program_statement, combinator::eof)
        .map(|v| v.0)
        .parse_next(input)?;

    parse_whitespace(input)?;

    if input.len() > 0 {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    Ok(Program { statements })
}
