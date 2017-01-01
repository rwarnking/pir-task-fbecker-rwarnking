use std::env;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::fs::File;

pub enum CpErrors {
    CouldNotReadInputParameters,
    CouldNotOpenSourceFile(String),
    CouldNotWriteToOutputFile(String)
}

/// Copy the contents of one file to another
pub fn mycp(from: &str, to: &str) {

    let f = match File::open(from) {
        Ok(v) => v,
        Err(_) => return error_handling(CpErrors::CouldNotOpenSourceFile(from.to_owned())),
    };

    let t = match File::open(to) {
        Ok(v) => v,
        Err(_) => File::create(to).unwrap(),
    };

    let read_buf = BufReader::new(f);
    let mut write_buf = LineWriter::new(t);

    for line in read_buf.lines() {
        match write_buf.write_fmt(format_args!("{}\n", line.unwrap_or(String::new()))) {
            Ok(_) => {},
            Err(_) => error_handling(CpErrors::CouldNotWriteToOutputFile(to.to_owned())),
        }
    }
}

/// Prints error messages for error I can't handle
fn error_handling(err: CpErrors) {
    match err {
        CpErrors::CouldNotReadInputParameters =>
            println!("It was not possible to read the input parameters."),
        CpErrors::CouldNotOpenSourceFile(s) =>
            println!("Source file \'{}\' could not be opened.", s),
        CpErrors::CouldNotWriteToOutputFile(s) =>
            println!("Writing to the output file \'{}\' failed.", s),
    }
}

/// Handles input from the command line
pub fn get_command_input() -> Option<(String, String)> {
    let mut from = String::new();
    let mut input = env::args();

    for index in 0..3 {
        if let Some(elem) = input.next() {
            match index {
                1 => from = elem,
                2 => return Some((from, elem)),
                _ => {}
            }
        }
    }
    error_handling(CpErrors::CouldNotReadInputParameters);

    None
}
