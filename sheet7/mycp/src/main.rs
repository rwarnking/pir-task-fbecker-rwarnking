use std::env;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::fs::File;

#[allow(dead_code)]
enum CpErrors {
    SomethingHappened,
    TooManyArguments,
    CouldNotOpenSourceFile,
    CouldNotWriteToOutputFile
}

fn main() {
    match get_command_input() {
        Ok((from, to)) => mycp(&from, &to),
        Err(e) => error_handling(e),
    }
}

/// Copy one file to another
///
/// I tried to use the try!() macro but always got a
/// compiler error saying an error occured outside of my
/// code inside the macro.
fn mycp(from: &str, to: &str) {
    //
    let f = match File::open(from) {
        Ok(v) => v,
        Err(_) => return error_handling(CpErrors::CouldNotOpenSourceFile),
    };

    let t = match File::open(to) {
        Ok(v) => v,
        Err(_) => File::create(to).unwrap(),
    };

    let read_buf = BufReader::new(f);
    let mut write_buf = LineWriter::new(t);

    for line in read_buf.lines() {
        match write_buf.write_fmt(format_args!("{}\n", line.unwrap())) {
            Ok(_) => {},
            Err(_) => error_handling(CpErrors::CouldNotWriteToOutputFile),
        }
    }
}

/// Prints error messages for error I can't handle
fn error_handling(err: CpErrors) {
    match err {
        CpErrors::SomethingHappened => println!("I think somethin happened."),
        CpErrors::TooManyArguments => println!("Too many command line arguments."),
        CpErrors::CouldNotOpenSourceFile => println!("Source file could not be opened."),
        CpErrors::CouldNotWriteToOutputFile => println!("Writing to the output file failed."),
    }
}

/// Handles input from the command line
fn get_command_input() -> Result<(String, String), CpErrors>{
    let mut elem_count = 0;
    let mut from = String::new();

    for elem in env::args() {
        if elem.contains("mycp") {
            elem_count = 1;
        } else {
            match elem_count {
                0 | 1 => { elem_count = 2; from = elem; },
                2 => return Ok((from, elem)),
                _ => return Err(CpErrors::TooManyArguments)
            }
        }
    }

    Err(CpErrors::SomethingHappened)
}

