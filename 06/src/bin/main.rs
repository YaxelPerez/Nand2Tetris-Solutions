
use std::io::prelude::*;
use std::io::LineWriter;
use std::fs;
use std::path::PathBuf;
#[macro_use] extern crate clap;

extern crate hack;

fn main() {
    // argument parsing
    let matches = clap_app!(app =>
        (version: "0.1")
        (@arg INPUT: <FILE> "Input file")
        (@arg OUTPUT: -o --output [FILE] "Output file" )
    ).get_matches();

    let input_path = PathBuf::from(matches.value_of("INPUT").unwrap());

    if !input_path.exists() {
        panic!("File {} does not exist!", input_path.display());
    }

    let output_path = match matches.value_of("OUTPUT") {
        Some(filename) => PathBuf::from(filename),
        None => input_path.with_extension("hack")
    };

    println!("Assembling {}...", input_path.display());

    let input_contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");

    let mut program = hack::Program::from_string(input_contents);
    program.resolve_symbols();
    let words = program.to_bytes(); // confusing because I forgot words != bytes...

    let out_file = fs::File::create(output_path).unwrap();
    let mut out_file = LineWriter::new(out_file);

    for word in words.iter() {
        out_file.write_all(format!("{:016b}\n", word).as_bytes()).unwrap();
    }
    out_file.flush().unwrap();
}
