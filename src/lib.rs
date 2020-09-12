extern crate env_logger;
#[macro_use]
extern crate log;

#[derive(Debug, PartialEq)]
pub enum Error {
    Parser(parser::Error),
    Eval(tree::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod parser;
pub mod tokens;
pub mod tree;

impl std::convert::From<parser::Error> for Error {
    fn from(error: parser::Error) -> Error {
        Error::Parser(error)
    }
}

impl std::convert::From<tree::Error> for Error {
    fn from(error: tree::Error) -> Error {
        Error::Eval(error)
    }
}

pub use self::parser::parse;

use std::collections::HashMap;

pub fn eval(expr: &str, map: &HashMap<String, f64>) -> Result<f64> {
    parse(expr)?.eval(map).map_err(|err| err.into())
}
