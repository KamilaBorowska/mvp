extern crate mvp;

use mvp::parser::ast::{BinaryOperator, Expression, Number, NumberWidth, Opcode, OpcodeMode,
                       Statement};
use mvp::parser::grammar::{statement, IResult};

fn opcode(width: Option<u32>, mode: OpcodeMode) -> Statement {
    Statement::Opcode(Opcode {
                          name: String::from("LDA").into_boxed_str(),
                          width: width,
                          mode: mode,
                          value: Expression::Number(Number {
                                                        value: 19,
                                                        width: NumberWidth::None,
                                                    }),
                      })
}

#[test]
fn address() {
    let input = "LDA 19 :";
    let result = statement(input);
    let expected = IResult::Done(":", opcode(None, OpcodeMode::Address));
    assert_eq!(result, expected);
}

#[test]
fn indirect() {
    let input = "LDA (19) :";
    let result = statement(input);
    let expected = IResult::Done(":", opcode(None, OpcodeMode::Indirect));
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
                name: String::from("LDA").into_boxed_str(),
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
    let input = "LDA ( $19 ) + 2 :";
    let result = statement(input);
    let expected = IResult::Done(
        ":",
        Statement::Opcode(
            Opcode {
                name: String::from("LDA").into_boxed_str(),
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
    let input = "LDA # 19";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::Immediate));
    assert_eq!(result, expected);
}

#[test]
fn opcode_width() {
    let input = "LDA.w # ( 19 )";
    let result = statement(input);
    let expected = IResult::Done("", opcode(Some(2), OpcodeMode::Immediate));
    assert_eq!(result, expected);
}

#[test]
fn uppercase_opcode_width() {
    let input = "LDA.W # (19)";
    let result = statement(input);
    let expected = IResult::Done("", opcode(Some(2), OpcodeMode::Immediate));
    assert_eq!(result, expected);
}

#[test]
fn opcode_width_with_spaces() {
    let input = "LDA . w #19";
    let result = statement(input);
    let expected = IResult::Done("", opcode(Some(2), OpcodeMode::Immediate));
    assert_eq!(result, expected);
}

#[test]
fn x_address() {
    let input = "LDA 19,x";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::XAddress));
    assert_eq!(result, expected);
}

#[test]
fn case_insensitive_x_address() {
    let input = "LDA 19 , X";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::XAddress));
    assert_eq!(result, expected);
}

#[test]
fn y_address() {
    let input = "LDA 19 , y ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::YAddress));
    assert_eq!(result, expected);
}

#[test]
fn stack_address() {
    let input = " LDA 19    ,    s  ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::StackAddress));
    assert_eq!(result, expected);
}

#[test]
fn x_indirect() {
    let input = "LDA ( 19 , x ) ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::XIndirect));
    assert_eq!(result, expected);
}

#[test]
fn indirect_y() {
    let input = " LDA ( 19 ) , y ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::IndirectY));
    assert_eq!(result, expected);
}

#[test]
fn stack_indirect_y() {
    let input = " LDA ( 19 , s ) , y ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::StackIndirectY));
    assert_eq!(result, expected);
}

#[test]
fn case_insensitive_stack_indirect_y() {
    let input = " LDA ( 19 , S ) , Y ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::StackIndirectY));
    assert_eq!(result, expected);
}

#[test]
fn long_indirect() {
    let input = " LDA [ 19 ] :";
    let result = statement(input);
    let expected = IResult::Done(":", opcode(None, OpcodeMode::LongIndirect));
    assert_eq!(result, expected);
}

#[test]
fn long_indirect_y() {
    let input = " LDA [ 19 ] , y ";
    let result = statement(input);
    let expected = IResult::Done("", opcode(None, OpcodeMode::LongIndirectY));
    assert_eq!(result, expected);
}

#[test]
fn move_mode() {
    let input = " LDA 19 , 2 ";
    let result = statement(input);
    let second = Expression::Number(Number {
                                        value: 2,
                                        width: NumberWidth::None,
                                    });
    let expected = IResult::Done("", opcode(None, OpcodeMode::Move { second: second }));
    assert_eq!(result, expected);
}
