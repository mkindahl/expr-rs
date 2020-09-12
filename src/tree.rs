use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoValue(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum ExprTree {
    Var(String),
    Float(f64),
    Add(Box<ExprTree>, Box<ExprTree>),
    Sub(Box<ExprTree>, Box<ExprTree>),
    Mul(Box<ExprTree>, Box<ExprTree>),
    Div(Box<ExprTree>, Box<ExprTree>),
}

impl ExprTree {
    pub fn eval(self, map: &HashMap<String, f64>) -> Result<f64> {
        match self {
            ExprTree::Float(num) => Ok(num),
            ExprTree::Var(name) => map
                .get(&name)
                .ok_or(Error::NoValue(name.clone()))
                .map(Clone::clone),
            ExprTree::Add(lhs, rhs) => Ok(lhs.eval(map)? + rhs.eval(map)?),
            ExprTree::Sub(lhs, rhs) => Ok(lhs.eval(map)? - rhs.eval(map)?),
            ExprTree::Mul(lhs, rhs) => Ok(lhs.eval(map)? * rhs.eval(map)?),
            ExprTree::Div(lhs, rhs) => Ok(lhs.eval(map)? / rhs.eval(map)?),
        }
    }
}
