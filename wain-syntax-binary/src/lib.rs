#![deny(unsafe_code)]

mod error;
mod leb128;
mod parser;
mod source;

pub use error::{Error, ErrorKind, Result};
pub use parser::Parser;
use source::BinarySource;
use wain_ast::Root;

pub fn parse<'s>(input: &'s [u8]) -> Result<'s, Root<'s, BinarySource>> {
    let mut parser = Parser::new(input);
    parser.parse()
}
