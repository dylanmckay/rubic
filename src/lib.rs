pub use self::errors::{Error, ErrorKind};

pub mod errors;

pub mod ast;
pub mod parse;
pub mod ir;

#[macro_use]
extern crate error_chain;
