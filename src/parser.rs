use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct ParserModuleVariableData {
    pub name: String,
    pub width: KnownBitWidth,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ParserModule {
    name: String,
    variables: Vec<ParserModuleVariable>,
}

#[derive(Debug)]
pub struct ParserModuleInOut {
    pub inputs: Vec<ParserModuleVariableData>,
    pub outputs: Vec<ParserModuleVariableData>,
}

#[derive(Debug)]
pub struct ParserState {
    modules_stack: Vec<ParserModule>,
    all_modules: HashMap<String, ParserModuleInOut>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            modules_stack: vec![ParserModule::new(String::from("$"))],
            all_modules: HashMap::new(),
        }
    }

    pub fn start_new_module(&mut self, name: String) {
        self.modules_stack.push(ParserModule::new(name));
    }

    pub fn current_module(&mut self) -> &mut ParserModule {
        self.modules_stack.last_mut().unwrap()
    }

    pub fn add_variable(&mut self, variable: ParserModuleVariable) {
        self.current_module().add_variable(variable);
    }

    pub fn find_variable(&self, name: &str) -> Option<&ParserModuleVariable> {
        for module in self.modules_stack.iter().rev() {
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

    pub fn find_module(&self, name: &str) -> Option<&ParserModuleInOut> {
        self.all_modules.get(name)
    }

    pub fn end_current_module(&mut self) -> ParserModule {
        let module = self.modules_stack.pop().unwrap();

        let mut inputs = vec![];
        let mut outputs = vec![];

        for variable in module.variables.iter() {
            match variable {
                ParserModuleVariable::Input(data) => inputs.push(data.clone()),
                ParserModuleVariable::Output(data) => outputs.push(data.clone()),
                ParserModuleVariable::Wire(_) => {}
            }
        }

        self.all_modules
            .insert(module.name.clone(), ParserModuleInOut { inputs, outputs });

        module
    }
}

impl ParserModule {
    pub fn new(name: String) -> Self {
        Self {
            variables: Vec::new(),
            name,
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

    let ret = program::parse_program(&mut stream);
    println!("{:#?}", stream.state);
    ret
}
