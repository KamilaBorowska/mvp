extern crate mvp;

use mvp::parser::ast::{BinaryOperator, Expression, Label, Number, NumberWidth, VariableName};
use mvp::parser::grammar::{self, CompleteStr};

macro_rules! binary_op {
    (+) => {
        BinaryOperator::Add
    };
    (-) => {
        BinaryOperator::Sub
    };
    (*) => {
        BinaryOperator::Mul
    };
    (/) => {
        BinaryOperator::Div
    };
    ($ignore:tt) => {
        unreachable!()
    };
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
                let items = (args[0].clone(), args[1].clone());
                Expression::Binary(binary_op!($f), Box::new(items))
            }
            name => Expression::Call(VariableName(name), args),
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

macro_rules! test {
    ($name:ident : $input:expr => $token:tt) => {
        #[test]
        fn $name() {
            let input = CompleteStr($input);
            let result = grammar::expression(input);
            let expected = Ok((CompleteStr(""), tree!($token)));
            assert_eq!(result, expected);
        }
    };
}

test!(addition: "2 + 3 + 4" => (+ (+ 2 3) 4));
test!(multiplication: "20 * 30 / 40" => (/ (* 20 30) 40));
test!(precedence: "2 + 3 * 4 - 5 / 6 + 7" => (+ (- (+ 2 (* 3 4)) (/ 5 6)) 7));
test!(parens: " ( 2 + 3 ) * 4 " => (* (+ 2 3) 4));
test!(call: " sqrt ( 42 ) " => (sqrt 42));
test!(complex_calls: "f(1, 8 + g(2, 3) + 9, 4) * 2" => (* (f 1 (+ (+ 8 (g 2 3)) 9) 4) 2));
test!(hex_digits: " $ Fe " => (one 0xFE));
test!(two_byte_hex_digits: " $ FeDc " => (two 0xFEDC));
test!(invalid_hex_digit_size: " $ FeD " => 0xFED);

#[test]
fn reject_huge_numbers() {
    let input = CompleteStr("2859421875392683928732568");
    let result = grammar::expression(input);
    assert!(result.is_err());
}

#[test]
fn no_function_call_tuples() {
    let input = CompleteStr("f((1, 2))");
    let result = grammar::expression(input);
    let expected = Ok((
        CompleteStr("((1, 2))"),
        Expression::Variable(Label::Named(VariableName("f"))),
    ));
    assert_eq!(result, expected);
}

#[test]
fn hex_digits_cannot_have_spaces() {
    let input = CompleteStr(" $ FE DC ");
    let result = grammar::expression(input);
    assert_eq!(result, Ok((CompleteStr("DC "), tree!(one 0xFE))));
}

#[test]
fn label_math() {
    let input = CompleteStr("+ + ++");
    let result = grammar::expression(input);
    assert_eq!(
        result,
        Ok((
            CompleteStr(""),
            Expression::Binary(
                BinaryOperator::Add,
                Box::new((
                    Expression::Variable(Label::Relative(1)),
                    Expression::Variable(Label::Relative(2))
                ))
            )
        ))
    )
}
