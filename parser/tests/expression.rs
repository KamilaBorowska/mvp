#![feature(trace_macros)]

extern crate mvp_parser;

use mvp_parser::ast::{BinaryOperator, Expression, Number, NumberWidth, VariableName};
use mvp_parser::parser::{self, IResult};

macro_rules! binary_op {
    (+) => { BinaryOperator::Add };
    (-) => { BinaryOperator::Sub };
    (*) => { BinaryOperator::Mul };
    (/) => { BinaryOperator::Div };
    ($ignore:tt) => { unreachable!() };
}

macro_rules! tree_meta {
    ((one $number:expr)) => {
        Expression::Number(Number {
            value: $number,
            width: NumberWidth::OneByte,
        })
    };
    ((two $number:expr)) => {
        Expression::Number(Number {
            value: $number,
            width: NumberWidth::TwoBytes,
        })
    };
    (($f:tt $($arg:tt)*)) => {{
        let args = vec![$(tree_meta!($arg)),*];
        #[allow(unreachable_code, unused_variables)]
        match stringify!($f) {
            "+"|"-"|"*"|"/" => {
                // Expression::Binary expects two arguments, but the macro can be expanded
                // even when there is more.
                let items = [args[0].clone(), args[1].clone()];
                Expression::Binary(binary_op!($f), Box::new(items))
            }
            name => Expression::Call(VariableName(String::from(name)), args),
        }
    }};
    ($number:expr) => {
        Expression::Number(Number {
            value: $number,
            width: NumberWidth::None,
        })
    };
}

macro_rules! tree {
    ($token:tt) => {
        tree_meta!($token);
    };
    ($($token:tt)*) => {
        tree_meta!(($($token)*));
    };
}

#[test]
fn addition() {
    let input = "2 + 3 + 4";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(+ (+ 2 3) 4));
    assert_eq!(result, expected);
}


#[test]
fn multiplication() {
    let input = "20 * 30 / 40";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(/ (* 20 30) 40));
    assert_eq!(result, expected);
}

#[test]
fn precedence() {
    let input = "2 + 3 * 4 - 5 / 6 + 7";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(+ (- (+ 2 (* 3 4)) (/ 5 6)) 7));
    assert_eq!(result, expected);
}

#[test]
fn parens() {
    let input = " ( 2 + 3 ) * 4 ";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(* (+ 2 3) 4));
    assert_eq!(result, expected);
}

#[test]
fn reject_huge_numbers() {
    let input = "2859421875392683928732568";
    let result = parser::expression(input);
    assert!(result.is_err());
}

#[test]
fn call() {
    let input = " sqrt ( 42 ) ";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(sqrt 42));
    assert_eq!(result, expected);
}

#[test]
fn complex_calls() {
    let input = "f(1, 8 + g(2, 3) + 9, 4) * 2";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(* (f 1 (+ (+ 8 (g 2 3)) 9) 4) 2));
    assert_eq!(result, expected);
}

#[test]
fn no_function_call_tuples() {
    let input = "f((1, 2))";
    let result = parser::expression(input);
    assert!(result.is_err());
}

#[test]
fn hex_digits() {
    let input = " $ Fe ";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(one 0xFE));
    assert_eq!(result, expected);
}

#[test]
fn two_byte_hex_digits() {
    let input = " $ FeDc ";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(two 0xFEDC));
    assert_eq!(result, expected);
}

#[test]
fn invalid_hex_digit_size() {
    let input = " $ FeD ";
    let result = parser::expression(input);
    let expected = IResult::Done("", tree!(0xFED));
    assert_eq!(result, expected);
}

#[test]
fn hex_digits_cannot_have_spaces() {
    let input = " $ FE DC ";
    let result = parser::expression(input);
    assert_eq!(result, IResult::Done("DC ", tree!(one 0xFE)));
}
