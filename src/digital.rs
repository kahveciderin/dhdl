use std::{collections::HashMap, sync::Arc};

use rand::Rng;
use xmlwriter::XmlWriter;

use crate::types::module::{ExternalModule, Module};

mod expression;
mod module;
mod program;
mod variable_definition;

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

static mut CURRENT_COORDINATE: Coordinate = Coordinate { x: 0, y: 0 };
static mut LOOP_COUNT: i64 = 0;
impl Coordinate {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        w.write_attribute("x", &self.x.to_string());
        w.write_attribute("y", &self.y.to_string());
    }

    pub fn next() -> Self {
        let mut rng = rand::thread_rng();

        unsafe {
            // todo: make this look a bit nicer
            let ret = CURRENT_COORDINATE.clone();

            // we are incrementing both on purpose, to make sure wires
            // somehow don't overlap
            CURRENT_COORDINATE.x += 500 + rng.gen_range(-5..=5) * 20;
            CURRENT_COORDINATE.y += 300+ rng.gen_range(-5..=5) * 20;

            if CURRENT_COORDINATE.x > 5000 {
                CURRENT_COORDINATE.x = (LOOP_COUNT * 20) % 500;
                LOOP_COUNT += 1;
            }
            ret
        }
    }

    pub fn add(&self, x: i64, y: i64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

pub struct Wire {
    start: Coordinate,
    end: Coordinate,
}

impl Wire {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        w.start_element("wire");
        w.start_element("p1");
        self.start.to_xml(w);
        w.end_element();

        w.start_element("p2");
        self.end.to_xml(w);
        w.end_element();

        w.end_element();
    }
}

#[derive(Debug, Clone)]
pub enum EntryValueDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum EntryValue {
    String(String),
    Integer(i32),
    Long(i64),
    Boolean(bool),
    Color((u8, u8, u8, u8)),
    Direction(EntryValueDirection),
    Data(String),
}

impl EntryValue {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        match self {
            EntryValue::String(s) => {
                w.start_element("string");
                w.write_text(s);
                w.end_element();
            }
            EntryValue::Data(s) => {
                w.start_element("data");
                w.write_text(s);
                w.end_element();
            }
            EntryValue::Integer(i) => {
                w.start_element("int");
                w.write_text(&i.to_string());
                w.end_element();
            }
            EntryValue::Long(l) => {
                w.start_element("long");
                w.write_text(&l.to_string());
                w.end_element();
            }
            EntryValue::Boolean(b) => {
                w.start_element("boolean");
                w.write_text(if *b { "true" } else { "false" });
                w.end_element();
            }
            EntryValue::Color((r, g, b, a)) => {
                w.start_element("awt-color");

                w.start_element("red");
                w.write_text(&r.to_string());
                w.end_element();
                w.start_element("green");
                w.write_text(&g.to_string());
                w.end_element();
                w.start_element("blue");
                w.write_text(&b.to_string());
                w.end_element();
                w.start_element("alpha");
                w.write_text(&a.to_string());
                w.end_element();

                w.end_element();
            }
            EntryValue::Direction(d) => {
                w.start_element("direction");
                w.write_text(match d {
                    EntryValueDirection::Up => "up",
                    EntryValueDirection::Down => "down",
                    EntryValueDirection::Left => "left",
                    EntryValueDirection::Right => "right",
                });
                w.end_element();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub value: EntryValue,
}

impl Entry {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        w.start_element("entry");
        EntryValue::String(self.name.clone()).to_xml(w);
        self.value.to_xml(w);
        w.end_element();
    }
}

#[derive(Debug)]
pub struct VisualElement {
    name: String,
    attributes: Vec<Entry>,
    position: Coordinate,
}

impl VisualElement {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        w.start_element("visualElement");

        w.start_element("elementName");
        w.write_text(&self.name);
        w.end_element();

        w.start_element("elementAttributes");
        for attribute in &self.attributes {
            attribute.to_xml(w);
        }
        w.end_element();

        w.start_element("pos");
        self.position.to_xml(w);
        w.end_element();

        w.end_element();
    }
}

#[derive(Debug, Clone)]
pub struct CircuitVariable {
    name: String,
    data: DigitalData,
    undefined: bool,
}

#[derive(Debug, Clone)]
pub enum CircuitModule {
    Internal(Module),
    External(ExternalModule),
}

impl CircuitModule {
    pub fn get_name(&self) -> String {
        match self {
            CircuitModule::Internal(module) => module.name.clone(),
            CircuitModule::External(module) => module.name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CurrentModule {
    variables: Vec<CircuitVariable>,
}

pub struct Circuit {
    wires: Vec<Wire>,
    visual_elements: Vec<VisualElement>,

    modules: Vec<CircuitModule>,

    current_module: Vec<CurrentModule>,
}

#[derive(Debug, Clone)]
pub enum DigitalData {
    Empty,
    Wire(u32, Coordinate),
    Object(HashMap<String, Arc<DigitalData>>),
}

impl DigitalData {
    pub fn get_size(&self) -> u32 {
        match self {
            DigitalData::Empty => panic!("Empty data has no size"),
            DigitalData::Wire(size, _) => *size,
            DigitalData::Object(map) => {
                if map.keys().len() != 1 {
                    panic!("! Object width has more than one key");
                }

                map.values().next().unwrap().as_ref().get_size()
            }
        }
    }
    pub fn get_position(&self) -> Coordinate {
        match self {
            DigitalData::Empty => panic!("Empty data has no position"),
            DigitalData::Wire(.., position) => position.clone(),
            DigitalData::Object(map) => {
                if map.keys().len() != 1 {
                    panic!("Object width has more than one key");
                }

                map.values().next().unwrap().as_ref().get_position()
            }
        }
    }
}

pub trait ToDigital {
    // returns wire positions
    fn convert_to_digital(&self, circuit: &mut Circuit) -> DigitalData;
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            wires: vec![],
            visual_elements: vec![],

            modules: vec![],

            current_module: vec![CurrentModule { variables: vec![] }],
        }
    }

    pub fn is_top(&self) -> bool {
        self.current_module.len() <= 1
    }

    pub fn add_variable(&mut self, variable: CircuitVariable) {
        self.current_module
            .last_mut()
            .unwrap()
            .variables
            .push(variable);
    }

    pub fn find_variable(&self, name: String) -> Option<&CircuitVariable> {
        for module in self.current_module.iter().rev() {
            for variable in &module.variables {
                if variable.name == name {
                    return Some(variable);
                }
            }
        }

        None
    }

    pub fn find_module(&self, name: &str) -> Option<&CircuitModule> {
        self.modules.iter().find(|module| module.get_name() == name)
    }

    pub fn as_xml(&self) -> String {
        let mut w = XmlWriter::new(xmlwriter::Options {
            indent: xmlwriter::Indent::None,
            ..xmlwriter::Options::default()
        });
        w.write_declaration();

        w.start_element("circuit");

        w.start_element("version");
        w.write_text("2");
        w.end_element();

        w.start_element("attributes");
        w.end_element();

        w.start_element("visualElements");
        for visual_element in &self.visual_elements {
            visual_element.to_xml(&mut w);
        }
        w.end_element();

        w.start_element("wires");
        for wire in &self.wires {
            wire.to_xml(&mut w);
        }
        w.end_element();

        w.start_element("measurementOrdering");
        w.end_element();

        w.end_element();

        w.end_document()
    }
}
