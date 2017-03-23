extern crate mvp_parser;

use mvp_parser::ast::{BinaryOperator, Expression, Number, VariableName};
use mvp_parser::parser::{self, IResult};

fn number(number: u32) -> Box<Expression> {
    Box::new(Expression::Number(Number {
        value: number,
        width: None,
    }))
}

#[test]
fn addition() {
    let input = "2 + 3 + 4";
    let result = parser::expression(input);
    let addition = BinaryOperator::Add;
    let expected = IResult::Done("",
                                 Expression::Binary(addition,
                                                    Box::new(Expression::Binary(addition,
                                                                                number(2),
                                                                                number(3))),
                                                    number(4)));
    assert_eq!(result, expected);
}

#[test]
fn multiplication() {
    let input = "20 * 30 / 40";
    let result = parser::expression(input);
    let expected =
        IResult::Done("",
                      Expression::Binary(BinaryOperator::Div,
                                         Box::new(Expression::Binary(BinaryOperator::Mul,
                                                                     number(20),
                                                                     number(30))),
                                         number(40)));
    assert_eq!(result, expected);
}

#[test]
fn precedence() {
    // ((2 + (3 * 4)) - (5 / 6)) + 7
    let input = "2 + 3 * 4 - 5 / 6 + 7";
    let result = parser::expression(input);
    let expected = IResult::Done("", Expression::Binary(
        BinaryOperator::Add,
        Box::new(Expression::Binary(
            BinaryOperator::Sub,
            Box::new(Expression::Binary(
                BinaryOperator::Add,
                number(2),
                Box::new(Expression::Binary(
                    BinaryOperator::Mul,
                    number(3),
                    number(4),
                )),
            )),
            Box::new(Expression::Binary(
                BinaryOperator::Div,
                number(5),
                number(6),
            )),
        )),
        number(7),
    ));
    assert_eq!(result, expected);
}

#[test]
fn parens() {
    let input = " ( 2 + 3 ) * 4 ";
    let result = parser::expression(input);
    let expected =
        IResult::Done("",
                      Expression::Binary(BinaryOperator::Mul,
                                         Box::new(Expression::Binary(BinaryOperator::Add,
                                                                     number(2),
                                                                     number(3))),
                                         number(4)));
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
    let expected = IResult::Done("",
                                 Expression::Call(VariableName(String::from("sqrt")),
                                                  vec![*number(42)]));
    assert_eq!(result, expected);
}

#[test]
fn complex_calls() {
    let input = "f(1, 8 + g(2, 3) + 9, 4) * 2";
    let result = parser::expression(input);
    let expected = IResult::Done("", Expression::Binary(
        BinaryOperator::Mul,
        Box::new(Expression::Call(
            VariableName(String::from("f")),
            vec![
                Expression::Number(Number { value: 1, width: None }),
                Expression::Binary(
                    BinaryOperator::Add,
                    Box::new(Expression::Binary(
                        BinaryOperator::Add,
                        number(8),
                        Box::new(Expression::Call(
                            VariableName(String::from("g")),
                            vec![*number(2), *number(3)],
                        )),
                    )),
                    number(9),
                ),
                Expression::Number(Number { value: 4, width: None }),
            ],
        )),
        number(2),
    ));
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
    let expected = IResult::Done("",
                                 Expression::Number(Number {
                                     value: 0xFE,
                                     width: Some(2),
                                 }));
    assert_eq!(result, expected);
}
