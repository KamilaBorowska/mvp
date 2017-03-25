extern crate mvp_parser;

use mvp_parser::ast::{BinaryOperator, Expression, Number, NumberWidth, Opcode, OpcodeMode};
use mvp_parser::parser::{opcode, IResult};

#[test]
fn address() {
    let input = "LDA $19";
    let result = opcode(input);
    let expected = IResult::Done("",
                                 Opcode {
                                     name: String::from("LDA"),
                                     mode: OpcodeMode::Address,
                                     value: Expression::Number(Number {
                                         value: 0x19,
                                         width: NumberWidth::OneByte,
                                     }),
                                 });
    assert_eq!(result, expected);
}

#[test]
fn indirect() {
    let input = "LDA ($19)";
    let result = opcode(input);
    let expected = IResult::Done("",
                                 Opcode {
                                     name: String::from("LDA"),
                                     mode: OpcodeMode::Indirect,
                                     value: Expression::Number(Number {
                                         value: 0x19,
                                         width: NumberWidth::OneByte,
                                     }),
                                 });
    assert_eq!(result, expected);
}

#[test]
fn tricky_address() {
    let input = "LDA ($19)+2";
    let result = opcode(input);
    let expected = IResult::Done("",
                                 Opcode {
                                     name: String::from("LDA"),
                                     mode: OpcodeMode::Address,
                                     value:
                                         Expression::Binary(BinaryOperator::Add,
                                                            Box::new(Expression::Number(Number {
                                                                value: 0x19,
                                                                width: NumberWidth::OneByte,
                                                            })),
                                                            Box::new(Expression::Number(Number {
                                                                value: 2,
                                                                width: NumberWidth::None,
                                                            }))),
                                 });
    assert_eq!(result, expected);
}

#[test]
fn tricky_address_with_spaces() {
    let input = "LDA ( $ 19 ) + 2 ";
    let result = opcode(input);
    let expected = IResult::Done("",
                                 Opcode {
                                     name: String::from("LDA"),
                                     mode: OpcodeMode::Address,
                                     value:
                                         Expression::Binary(BinaryOperator::Add,
                                                            Box::new(Expression::Number(Number {
                                                                value: 0x19,
                                                                width: NumberWidth::OneByte,
                                                            })),
                                                            Box::new(Expression::Number(Number {
                                                                value: 2,
                                                                width: NumberWidth::None,
                                                            }))),
                                 });
    assert_eq!(result, expected);
}

#[test]
fn immediate() {
    let input = "LDA # $ 19";
    let result = opcode(input);
    let expected = IResult::Done("",
                                 Opcode {
                                     name: String::from("LDA"),
                                     mode: OpcodeMode::Immediate,
                                     value: Expression::Number(Number {
                                         value: 0x19,
                                         width: NumberWidth::OneByte,
                                     }),
                                 });
    assert_eq!(result, expected);
}
