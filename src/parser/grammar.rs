//! Grammar AST parser.
//!
//! Many methods in this module return `Result<(CompleteStr, O), nom::Err<CompleteStr>>`
//! as an argument. If the result is `Ok`, the variant contains a tuple where the first
//! argument is text left to parse, and second is retrieved AST value.
//! `Err` means that parse did fail.

use parser::ast::{BinaryOperator, Expression, Label, Number, NumberWidth, Opcode, OpcodeMode,
                  Statement, VariableName};

use std::str::{self, FromStr};

pub use nom::types::CompleteStr;
use nom::{self, ErrorKind};
use unicode_xid::UnicodeXID;

fn valid_identifier_first_character(result: char) -> bool {
    result == '!' || result == '_' || UnicodeXID::is_xid_start(result)
}

fn valid_later_character(c: char) -> bool {
    UnicodeXID::is_xid_continue(c) || c == '_'
}

const OPERATORS: &'static str = "+-*/";

/// An identifier parser.
///
/// It allows any Unicode identifier as specified by [Unicode Standard Annex #31:
/// Unicode Identifier and Pattern Syntax](http://www.unicode.org/reports/tr31/tr31-9.html).
/// In addition, it allows use of underscores in identifiers, as it is tradition
/// in C like programming languages, and ! at beginning (like xkas and Asar require,
/// but not MVP).
///
/// # Examples
///
/// Parsing an Unicode identifier.
///
/// ```
/// use mvp::parser::grammar::{self, CompleteStr};
///
/// let parsed = grammar::identifier(CompleteStr("世界"));
/// assert_eq!(parsed, Ok((CompleteStr(""), "世界")));
/// ```
pub fn identifier(input: CompleteStr) -> Result<(CompleteStr, &str), nom::Err<CompleteStr>> {
    let mut indices = input.char_indices();
    match indices.next() {
        Some((_, c)) if valid_identifier_first_character(c) => {}
        _ => return Err(nom::Err::Error(error_position!(input, ErrorKind::Alpha))),
    };
    for (pos, c) in indices {
        if !valid_later_character(c) {
            return Ok((CompleteStr(&input[pos..]), &input[..pos]));
        }
    }
    Ok((CompleteStr(""), &input))
}

named!(pub statement<CompleteStr, Statement>, ws!(alt!(
    opcode => { |opcode| Statement::Opcode(opcode) }
)));

named!(immediate<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    char!('#') >>
    expression: expression >>
    (expression, OpcodeMode::Immediate)
)));

named!(indirect<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    char!('(') >>
    expression: expression >>
    char!(')') >>
    not!(one_of!(OPERATORS)) >>
    (expression, OpcodeMode::Indirect)
)));

named!(x_indirect<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    char!('(') >>
    expression: expression >>
    char!(',') >>
    tag_no_case!("x") >>
    char!(')') >>
    (expression, OpcodeMode::XIndirect)
)));

named!(indirect_y<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    char!('(') >>
    expression: expression >>
    char!(')') >>
    char!(',') >>
    tag_no_case!("y") >>
    (expression, OpcodeMode::IndirectY)
)));

named!(stack_indirect_y<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    char!('(') >>
    expression: expression >>
    char!(',') >>
    tag_no_case!("s") >>
    char!(')') >>
    char!(',') >>
    tag_no_case!("y") >>
    (expression, OpcodeMode::StackIndirectY)
)));

named!(long_indirect<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    char!('[') >>
    expression: expression >>
    char!(']') >>
    (expression, OpcodeMode::LongIndirect)
)));

named!(long_indirect_y<CompleteStr, (Expression, OpcodeMode)>, ws!(do_parse!(
    res: long_indirect >>
    char!(',') >>
    one_of!("yY") >>
    (res.0, OpcodeMode::LongIndirectY)
)));

named!(address<CompleteStr, (Expression, OpcodeMode)>, do_parse!(
    first: expression >>
    second: opt!(preceded!(char!(','), expression)) >>
    (first, match second {
        Some(second) => OpcodeMode::Move { second },
        None => OpcodeMode::Address,
    })
));

named!(opcode<CompleteStr, Opcode>, ws!(do_parse!(
    opcode: identifier >>
    width: opt!(preceded!(char!('.'), alt!(
        tag_no_case!("b") => {|_| 1}
        | tag_no_case!("w") => {|_| 2}
        | tag_no_case!("l") => {|_| 3}
    ))) >>
    result: alt!(
        indirect_y
        | indirect
        | x_indirect
        | address
        | immediate
        | long_indirect_y
        | long_indirect
        | stack_indirect_y
    ) >>
    (Opcode {
        name: &opcode,
        width: width,
        value: result.0,
        mode: result.1,
    })
)));

named!(
/// Assignment statement parser.
///
/// It expects variable name, followed by `=` character, and an expression
/// which marks expression to be stored as value.
///
/// # Examples
///
/// ```
/// use mvp::parser::grammar::{self, CompleteStr};
/// use mvp::parser::ast::{Expression, Number, NumberWidth, Statement, VariableName};
///
/// let parsed = grammar::assignment(CompleteStr("hello = 44"));
/// let expected = Statement::Assignment(
///     VariableName("hello"),
///     Expression::Number(Number { value: 44, width: NumberWidth::None }),
/// );
/// assert_eq!(parsed, Ok((CompleteStr(""), expected)));
/// ```
,
pub assignment<CompleteStr, Statement>, ws!(do_parse!(
    name: identifier >>
    char!('=') >>
    value: expression >>
    (Statement::Assignment(VariableName(&name), value))
)));

named!(label<CompleteStr, Label>, map!(identifier, |name| Label::Named(VariableName(&name))));

named!(
/// An expression parser.
///
/// This can be used as math expression parser, however due to language
/// limitations, it doesn't support types like decimal numbers.
/// However, it does support mathematical operators like addition,
/// subtraction, multiplication and division, as well as parenthesis.
///
/// # Example
///
/// Parsing a mathematical expression:
///
/// ```
/// use mvp::parser::grammar::{self, CompleteStr};
/// use mvp::parser::ast::{BinaryOperator, Expression, Number, NumberWidth};
///
/// let parsed = grammar::expression(CompleteStr("2 + 3"));
/// let expected = Ok((CompleteStr(""), Expression::Binary(
///     BinaryOperator::Add,
///     Box::new((
///         Expression::Number(Number { value: 2, width: NumberWidth::None }),
///         Expression::Number(Number { value: 3, width: NumberWidth::None }),
///     )),
/// )));
/// assert_eq!(parsed, expected);
/// ```
,
pub expression<CompleteStr, Expression>, ws!(do_parse!(
    init: term >>
    res: fold_many0!(
        pair!(alt!(
            char!('+') => {|_| BinaryOperator::Add}
            | char!('-') => {|_| BinaryOperator::Sub}
        ), term),
        init,
        |first, (operator, another)| {
            Expression::Binary(operator, Box::new((first, another)))
        }
    ) >>
    (res)
)));

named!(term<CompleteStr, Expression>, do_parse!(
    init: top_expression >>
    res: fold_many0!(
        pair!(alt!(
            char!('*') => {|_| BinaryOperator::Mul}
            | char!('/') => {|_| BinaryOperator::Div}
        ), top_expression),
        init,
        |first, (operator, another)| {
            Expression::Binary(operator, Box::new((first, another)))
        }
    ) >>
    (res)
));

named!(top_expression<CompleteStr, Expression>, alt!(
    paren_expression |
    number |
    hex_number |
    call |
    variable
));

named!(paren_expression<CompleteStr, Expression>, ws!(delimited!(char!('('), expression, char!(')'))));

named!(number<CompleteStr, Expression>, map!(
    map_res!(
        ws!(nom::digit),
        |x: CompleteStr| u32::from_str(&x)
    ),
    |value| Expression::Number(Number { value: value, width: NumberWidth::None })
));

fn hex_width_for_length(length: usize) -> NumberWidth {
    match length {
        2 => NumberWidth::OneByte,
        4 => NumberWidth::TwoBytes,
        _ => NumberWidth::None,
    }
}

named!(hex_number<CompleteStr, Expression>, ws!(do_parse!(
    char!('$') >>
    number: map!(
        map_res!(nom::hex_digit, |s: CompleteStr| u32::from_str_radix(&s, 16).map(|value| (s.len(), value))),
        |(length, value)| Expression::Number(Number {
            value: value,
            width: hex_width_for_length(length),
        })
    ) >>
    (number)
)));

named!(call<CompleteStr, Expression>, ws!(do_parse!(
    identifier: identifier >>
    parts: delimited!(
        char!('('),
        separated_list!(char!(','), expression),
        char!(')')
    ) >>
    (Expression::Call(VariableName(&identifier), parts))
)));

named!(variable<CompleteStr, Expression>, map!(label, Expression::Variable));
