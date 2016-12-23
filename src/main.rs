pub use self::errors::{Error, ErrorKind};

pub mod errors;

pub mod parse;

#[macro_use]
extern crate error_chain;

use std::{fs, env};
use std::io::prelude::*;

fn tokenize(file_path: &str) -> Result<(), Error> {
    println!("Tokenizing '{}'", file_path);

    let mut file = fs::File::open(file_path)?;

    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;

    let tokenizer = parse::Tokenizer::new(file_data.chars());

    for token in tokenizer {
        println!("{:?}", token);
    }

    Ok(())
}

fn main() {
    if let Some(file_path) = env::args().skip(1).next() {
        if let Err(e) = tokenize(&file_path) {
            println!("error: {}", e);
        }
    } else {
        println!("please enter an input file");
    }
}
