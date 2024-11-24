use xmlwriter::XmlWriter;

use crate::parser::datatype::KnownBitWidth;

mod expression;
mod program;
mod variable_definition;

#[derive(Debug, Clone)]
pub struct Coordinate {
    x: i64,
    y: i64,
}

static mut current_coordinate: Coordinate = Coordinate { x: 0, y: 0 };
impl Coordinate {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        w.write_attribute("x", &self.x.to_string());
        w.write_attribute("y", &self.y.to_string());
    }

    pub fn next() -> Self {
        unsafe {
            let ret = current_coordinate.clone();
            current_coordinate.x += 60;
            current_coordinate.y += 60;
            if current_coordinate.x > 2000 {
                current_coordinate.x = 0;
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

#[derive(Debug)]
pub enum EntryValue {
    String(String),
    Integer(i32),
    Long(i64),
    Boolean(bool),
}

impl EntryValue {
    pub fn to_xml(&self, w: &mut XmlWriter) {
        match self {
            EntryValue::String(s) => {
                w.start_element("string");
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
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    name: String,
    value: EntryValue,
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

pub struct CircuitVariable {
    name: String,
    position: Coordinate,
    width: KnownBitWidth,
}

pub struct Circuit {
    wires: Vec<Wire>,
    visual_elements: Vec<VisualElement>,

    variables: Vec<CircuitVariable>,
}

pub trait ToDigital {
    // returns wire positions
    fn convert_to_digital(&self, circuit: &mut Circuit) -> Vec<Coordinate>;
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            wires: vec![],
            visual_elements: vec![],

            variables: vec![],
        }
    }

    pub fn to_xml(self) -> String {
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
        for visual_element in self.visual_elements {
            visual_element.to_xml(&mut w);
        }
        w.end_element();

        w.start_element("wires");
        for wire in self.wires {
            wire.to_xml(&mut w);
        }
        w.end_element();

        w.start_element("measurementOrdering");
        w.end_element();

        w.end_element();

        w.end_document()
    }
}
