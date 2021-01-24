extern crate assert_matches;
extern crate expr;

use assert_matches::assert_matches;
use expr::eval;
use expr::parser::Error::*;
use expr::tokens::Token;
use expr::tree::Error::*;
use expr::Error::*;
use std::collections::HashMap;

#[test]
fn simple_expressions() {
    let map = HashMap::new();
    assert_eq!(eval("10", &map), Ok(10.0));
    assert_eq!(eval("10 + 10", &map), Ok(20.0));
    assert_eq!(eval("10 - 10", &map), Ok(0.0));
    assert_eq!(eval("10 * 10", &map), Ok(100.0));
    assert_eq!(eval("10 / 10", &map), Ok(1.0));
    assert_eq!(eval("10 + 2 * 3", &map), Ok(16.0));
    assert_eq!(eval("(10 + 2) * 3", &map), Ok(36.0));
    assert_eq!(eval("((10 + 2)) * 3", &map), Ok(36.0));
    assert_eq!(eval("2 / -4", &map), Ok(-0.5));

    assert_matches!(
        eval("10 + ", &map),
        Err(Parser(UnexpectedEndOfInput { rule: "factor", .. }))
    );
}

#[test]
fn variable_expressions() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), 12.0);
    assert_eq!(eval("10 + x", &map), Ok(22.0));
    assert_eq!(eval("x * 10", &map), Ok(120.0));
    assert_eq!(eval("(10-x)*3", &map), Ok(-6.0));
    assert_eq!(eval("-x*3", &map), Ok(-36.0));
    assert_eq!(eval("x*-3", &map), Ok(-36.0));
    assert_eq!(eval("x--3", &map), Ok(15.0));
    assert_eq!(eval("x-+3", &map), Ok(9.0));

    assert_eq!(
        eval("10 + x + y", &map),
        Err(Eval(NoValue("y".to_string())))
    );

    assert_matches!(
        eval("(10 + x", &map),
        Err(Parser(UnexpectedEndOfInput { rule: "factor", .. }))
    );

    assert_matches!(
        eval("((10 + x) * 2))", &map),
        Err(Parser(UnexpectedToken {
            token: Token::Close,
            rule: "expr",
            ..
        }))
    );
    assert_matches!(
        eval("x y", &map),
        Err(Parser(UnexpectedToken {
            token: Token::Symbol(_),
            rule: "expr",
            ..
        }))
    );
    assert_matches!(
        eval(")10 + x", &map),
        Err(Parser(UnexpectedToken {
            token: Token::Float(_),
            rule: "expr",
            ..
        }))
    );
    assert_matches!(
        eval("(10 + x", &map),
        Err(Parser(UnexpectedEndOfInput {
            expect: "')'",
            rule: "factor",
            ..
        }))
    );
    assert_matches!(
        eval("((10 + x)", &map),
        Err(Parser(UnexpectedEndOfInput {
            expect: "')'",
            rule: "factor",
            ..
        }))
    );
}
