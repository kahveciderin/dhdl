use datatype::{GetBitWidth, KnownBitWidth};
use winnow::{PResult, Stateful};

use crate::types::{decorator::BitWidth, program::Program};

mod argument;
pub mod datatype;
mod decorator;
mod expression;
mod identifier;
mod module;
mod number;
mod program;
mod trivial_tokens;
mod variable_definition;
mod whitespace;

#[derive(Debug)]
pub struct ParserModuleVariableData {
    pub name: String,
    pub width: KnownBitWidth,
}

#[derive(Debug)]
pub enum ParserModuleVariable {
    Input(ParserModuleVariableData),
    Output(ParserModuleVariableData),
    Wire(ParserModuleVariableData),
}

impl GetBitWidth for ParserModuleVariable {
    fn get_bit_width(&self, state: &ParserState) -> datatype::KnownBitWidth {
        match self {
            ParserModuleVariable::Input(data) => data.width.clone(),
            ParserModuleVariable::Output(data) => data.width.clone(),
            ParserModuleVariable::Wire(data) => data.width.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ParserModule {
    variables: Vec<ParserModuleVariable>,
}

#[derive(Debug)]
pub struct ParserState {
    modules: Vec<ParserModule>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            modules: vec![ParserModule::new()],
        }
    }

    pub fn start_new_module(&mut self) {
        self.modules.push(ParserModule::new());
    }

    pub fn current_module(&mut self) -> &mut ParserModule {
        self.modules.last_mut().unwrap()
    }

    pub fn add_variable(&mut self, variable: ParserModuleVariable) {
        self.current_module().add_variable(variable);
    }

    pub fn find_variable(&self, name: &str) -> Option<&ParserModuleVariable> {
        for module in self.modules.iter().rev() {
            for variable in &module.variables {
                match variable {
                    ParserModuleVariable::Input(data) => {
                        if data.name == name {
                            return Some(variable);
                        }
                    }
                    ParserModuleVariable::Output(data) => {
                        if data.name == name {
                            return Some(variable);
                        }
                    }
                    ParserModuleVariable::Wire(data) => {
                        if data.name == name {
                            return Some(variable);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn end_current_module(&mut self) -> ParserModule {
        self.modules.pop().unwrap()
    }
}

impl ParserModule {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, variable: ParserModuleVariable) {
        self.variables.push(variable);
    }
}

pub type Stream<'is> = Stateful<&'is str, ParserState>;

pub fn parse_program(input: &str) -> PResult<Program> {
    let mut stream = Stream {
        input,
        state: ParserState::new(),
    };

    program::parse_program(&mut stream)
}
