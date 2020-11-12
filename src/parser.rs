//! Predictive expression parse for expressions.

use tokens::{Token, Tokenizer};
use tree::ExprTree;

/// Parse expression.
///
/// Parse an expression from a string to produce an expression tree.
/// The grammar is given by the following rules:
///
/// expr ::= term (("+" | "-") term)*
/// term ::= factor (("*" | "/") factor)*
/// factor ::= number | variable | "(" expr ")"
///
/// # Returns
///
/// An expression tree
///
/// # Example
///
/// ```
/// # use expr::parse;
/// # use std::collections::HashMap;
/// let tree = parse("x + 10").unwrap();
/// let mut map = HashMap::new();
/// map.insert("x".to_string(), 12.0);
/// assert_eq!(tree.eval(&map), Ok(22.0));
/// ```
pub fn parse(text: &str) -> Result<ExprTree> {
    debug!("Starting parse");
    let mut tokens = Tokenizer::new(text);
    let tree = expr_rule(&mut tokens);
    let result = match tokens.next() {
        None => tree,
        Some(tok) => Err(Error::UnexpectedToken {
            token: tok,
            rule: "expr",
            expect: "end of input",
        }),
    };
    debug!("Exiting parse: {:?}", result);
    result
}

fn expr_rule(tokens: &mut Tokenizer) -> Result<ExprTree> {
    debug!("expr: enter");
    let mut tree = term_rule(tokens)?;
    while let Some(Token::Plus) | Some(Token::Minus) = tokens.peek("expr") {
        let tok = tokens.next();
        debug!("expr: read {:?}", tok);
        let rhs = term_rule(tokens)?;
        match tok {
            Some(Token::Plus) => {
                tree = ExprTree::Add(Box::new(tree), Box::new(rhs));
            }
            Some(Token::Minus) => {
                tree = ExprTree::Sub(Box::new(tree), Box::new(rhs));
            }
            Some(tok) => {
                return Err(Error::UnexpectedToken {
                    token: tok,
                    rule: "expr",
                    expect: "'+' or '-'",
                });
            }
            None => {
                return Err(Error::UnexpectedEndOfInput {
                    rule: "expr",
                    expect: "'+' or '-'",
                });
            }
        }
    }
    debug!("expr: leave");
    Ok(tree)
}

fn term_rule(tokens: &mut Tokenizer) -> Result<ExprTree> {
    debug!("term: enter");
    let mut tree = factor_rule(tokens)?;
    while let Some(Token::Star) | Some(Token::Slash) = tokens.peek("term") {
        let tok = tokens.next();
        debug!("term: read {:?}", tok);
        let rhs = factor_rule(tokens)?;
        match tok {
            Some(Token::Star) => {
                tree = ExprTree::Mul(Box::new(tree), Box::new(rhs));
            }
            Some(Token::Slash) => {
                tree = ExprTree::Div(Box::new(tree), Box::new(rhs));
            }
            Some(tok) => {
                return Err(Error::UnexpectedToken {
                    token: tok,
                    rule: "term",
                    expect: "'*' or '/'",
                });
            }
            None => {
                return Err(Error::UnexpectedEndOfInput {
                    rule: "term",
                    expect: "'*' or '/'",
                });
            }
        }
    }
    debug!("term: leave");
    Ok(tree)
}

fn factor_rule(tokens: &mut Tokenizer) -> Result<ExprTree> {
    debug!("factor: enter");
    let result = {
        let tok = tokens.next();
        debug!("factor: read {:?}", tok);
        match tok {
            Some(Token::Float(number)) => Ok(ExprTree::Float(number)),
            Some(Token::Symbol(name)) => Ok(ExprTree::Var(name)),
            Some(Token::Open) => {
                let expr = expr_rule(tokens)?;
                match tokens.next() {
                    Some(Token::Close) => Ok(expr),
                    Some(tok) => Err(Error::UnexpectedToken {
                        token: tok,
                        rule: "factor",
                        expect: "')'",
                    }),
                    None => Err(Error::UnexpectedEndOfInput {
                        rule: "factor",
                        expect: "')'",
                    }),
                }
            }
            Some(tok) => Err(Error::UnexpectedToken {
                token: tok,
                rule: "factor",
                expect: "number, variable, or '('",
            }),
            None => Err(Error::UnexpectedEndOfInput {
                rule: "factor",
                expect: "number, variable, or '('",
            }),
        }
    };
    debug!("factor: leave with {:?}", result);
    result
}

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEndOfInput {
        rule: &'static str,
        expect: &'static str,
    },
    UnexpectedToken {
        token: Token,
        rule: &'static str,
        expect: &'static str,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::UnexpectedEndOfInput {
                ref rule,
                ref expect,
            } => write!(
                f,
                "unexpected end of input when parsing {}, expected {}",
                rule, expect
            ),
            Error::UnexpectedToken {
                ref token,
                ref rule,
                ref expect,
            } => write!(
                f,
                "unexpected token '{}' when parsing {}, expected {}",
                token, rule, expect
            ),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::UnexpectedEndOfInput { .. } => "unexpected end of input",
            Error::UnexpectedToken { .. } => "unexpected token",
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::Error::*;
    use super::ExprTree::*;
    use super::Token;
    use super::{parse, ExprTree};

    fn check(expr: &str, tree: ExprTree) {
        assert_eq!(parse(expr), Ok(tree));
    }

    #[test]
    fn good_parse() {
        env_logger::init();

        assert_eq!(parse("10"), Ok(Float(10.0)));
        check("10+12", Add(Box::new(Float(10.0)), Box::new(Float(12.0))));
        check(
            "10+x",
            Add(Box::new(Float(10.0)), Box::new(Var("x".to_string()))),
        );
        check(
            "10+x*y",
            Add(
                Box::new(Float(10.0)),
                Box::new(Mul(
                    Box::new(Var("x".to_string())),
                    Box::new(Var("y".to_string())),
                )),
            ),
        );
        check(
            "10 + 12 * 20 - 2",
            Sub(
                Box::new(Add(
                    Box::new(Float(10.0)),
                    Box::new(Mul(Box::new(Float(12.0)), Box::new(Float(20.0)))),
                )),
                Box::new(Float(2.0)),
            ),
        );
    }

    #[test]
    fn bad_parse() {
        assert_matches!(
            parse("10 20"),
            Err(UnexpectedToken {
                token: Token::Float(num),
                rule: "expr",
                ..
            }) if num == 20.0
        );
        assert_matches!(
            parse("10++"),
            Err(UnexpectedToken {
                token: Token::Plus,
                rule: "factor",
                ..
            })
        );
        assert_matches!(
            parse("10+("),
            Err(UnexpectedEndOfInput { rule: "factor", .. })
        );
    }
}
