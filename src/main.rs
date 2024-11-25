use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use digital::ToDigital;
use parser::parse_program;

mod digital;
mod parser;
mod types;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);
    }

    let filename = &args[1];

    let input_file_path = format!("tests/{}.dhl", filename);
    let output_file_path = format!("output/{}.dig", filename);

    let mut file = File::open(input_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input = contents.as_str();

    let ast = parse_program(input).unwrap();

    // println!("{:#?}", ast);

    let mut circuit = digital::Circuit::new();
    ast.convert_to_digital(&mut circuit);

    let output = circuit.to_xml();

    let mut file = File::create(output_file_path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
