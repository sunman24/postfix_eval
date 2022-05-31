#[cfg(test)]

use std::f64::{INFINITY, NEG_INFINITY};

#[allow(unused)]
use crate::PostfixEvaluator as PE;

#[test]
fn ok_tests() {
    let mut pe = PE::new();

    assert_eq!(pe.eval("+1 -1 +"), Ok(0.0));
    assert_eq!(pe.eval("+1 -1 -  2 *"), Ok(4.0));
    assert_eq!(pe.eval("154 53 -"), Ok(101.0));
    assert_eq!(pe.eval("-154 -53 -"), Ok(-101.0));
    assert_eq!(pe.eval("1 1 /"), Ok(1.0));
    assert_eq!(pe.eval("155 5 /"), Ok(31.0));
    assert_eq!(pe.eval("-155 -5 /"), Ok(31.0));
    assert_eq!(pe.eval("155 -5 /"), Ok(-31.0));
    assert_eq!(pe.eval("1"), Ok(1.0));
}

#[test]
fn error_tests() {
    let mut pe = PE::new();

    assert_eq!(pe.eval("Hello world!"), Err("unknown token 'Hello'".to_string()));
    assert_eq!(pe.eval("+"), Err("the '+' operator requires two operands, but 0 was found".to_string()));
    assert_eq!(pe.eval("1 +"), Err("the '+' operator requires two operands, but 1 was found".to_string()));
    assert_eq!(pe.eval("*"), Err("the '*' operator requires two operands, but 0 was found".to_string()));
    assert_eq!(pe.eval("1 *"), Err("the '*' operator requires two operands, but 1 was found".to_string()));
    assert_eq!(pe.eval(""), Err("operand not found".to_string()));
}

#[test]
fn special_cases() {
    let mut pe = PE::new();

    assert_eq!(pe.eval("1 0 /"), Ok(INFINITY));
    assert_eq!(pe.eval("-1 0 /"), Ok(NEG_INFINITY));
    assert!(pe.eval("0 0 /").unwrap().is_nan());
}