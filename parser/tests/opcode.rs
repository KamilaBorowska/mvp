extern crate mvp_parser;

use mvp_parser::ast::{BinaryOperator, Expression, Number, NumberWidth, Opcode, OpcodeMode,
                      Statement};
use mvp_parser::parser::{statement, IResult};

#[test]
fn address() {
    let input = "LDA $19 :";
    let result = statement(input);
    let expected =
        IResult::Done(":",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: None,
                                            mode: OpcodeMode::Address,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn indirect() {
    let input = "LDA ($19) :";
    let result = statement(input);
    let expected =
        IResult::Done(":",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: None,
                                            mode: OpcodeMode::Indirect,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn tricky_address() {
    let input = "LDA ($19)+2 :";
    let result = statement(input);
    let expected = IResult::Done(
        ":",
        Statement::Opcode(
            Opcode {
                name: String::from("LDA"),
                width: None,
                mode: OpcodeMode::Address,
                value: Expression::Binary(
                    BinaryOperator::Add,
                    Box::new(
                        [
                            Expression::Number(
                                Number {
                                    value: 0x19,
                                    width: NumberWidth::OneByte,
                                },
                            ),
                            Expression::Number(
                                Number {
                                    value: 2,
                                    width: NumberWidth::None,
                                },
                            ),
                        ],
                    ),
                ),
            },
        ),
    );
    assert_eq!(result, expected);
}

#[test]
fn tricky_address_with_spaces() {
    let input = "LDA ( $ 19 ) + 2 :";
    let result = statement(input);
    let expected = IResult::Done(
        ":",
        Statement::Opcode(
            Opcode {
                name: String::from("LDA"),
                width: None,
                mode: OpcodeMode::Address,
                value: Expression::Binary(
                    BinaryOperator::Add,
                    Box::new(
                        [
                            Expression::Number(
                                Number {
                                    value: 0x19,
                                    width: NumberWidth::OneByte,
                                },
                            ),
                            Expression::Number(
                                Number {
                                    value: 2,
                                    width: NumberWidth::None,
                                },
                            ),
                        ],
                    ),
                ),
            },
        ),
    );
    assert_eq!(result, expected);
}

#[test]
fn immediate() {
    let input = "LDA # $ 19";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: None,
                                            mode: OpcodeMode::Immediate,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn opcode_width() {
    let input = "LDA.w # ( $ 19 )";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: Some(2),
                                            mode: OpcodeMode::Immediate,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn uppercase_opcode_width() {
    let input = "LDA.W # ($ 19)";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: Some(2),
                                            mode: OpcodeMode::Immediate,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn opcode_width_with_spaces() {
    let input = "LDA . w #$19";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: Some(2),
                                            mode: OpcodeMode::Immediate,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn x_address() {
    let input = "LDA $19,x";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: None,
                                            mode: OpcodeMode::XAddress,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn y_address() {
    let input = "LDA $ 19 , y ";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: None,
                                            mode: OpcodeMode::YAddress,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

#[test]
fn stack_address() {
    let input = " LDA $   19    ,    s  ";
    let result = statement(input);
    let expected =
        IResult::Done("",
                      Statement::Opcode(Opcode {
                                            name: String::from("LDA"),
                                            width: None,
                                            mode: OpcodeMode::StackAddress,
                                            value: Expression::Number(Number {
                                                                          value: 0x19,
                                                                          width:
                                                                              NumberWidth::OneByte,
                                                                      }),
                                        }));
    assert_eq!(result, expected);
}

