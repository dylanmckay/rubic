extern crate rubic;

use std::{fs, env};
use std::io::prelude::*;

fn parse(file_path: &str)
    -> Result<(), rubic::parse::Error> {
    println!("Tokenizing '{}'", file_path);

    let mut file = fs::File::open(file_path)?;

    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;

    let parser = rubic::parse::Parser::new(file_data.chars());
    let ast = parser.parse()?;

    println!("{:#?}", ast);

    Ok(())
}

fn main() {
    if let Some(file_path) = env::args().skip(1).next() {
        if let Err(e) = parse(&file_path) {
            println!("error: {}", e);
        }
    } else {
        println!("please enter an input file");
    }
}
