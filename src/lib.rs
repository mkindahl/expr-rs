extern crate env_logger;
#[macro_use]
extern crate log;
extern crate assert_matches;

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

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Parser(_) => "parser error",
            Error::Eval(_) => "eval error",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Parser(ref err) => write!(f, "parse error: {}", err),
            Error::Eval(ref err) => write!(f, "eval error: {}", err),
        }
    }
}

pub use self::parser::parse;

pub fn eval(expr: &str, map: &std::collections::HashMap<String, f64>) -> Result<f64> {
    parse(expr)?.eval(map).map_err(|err| err.into())
}
