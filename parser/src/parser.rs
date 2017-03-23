//! Grammar AST parser.
//!
//! Many methods in this module return `IResult<&str, O>` as an argument. It is
//! actually an enum defined by `nom` which represents parsing result.
//! `IResult::Done` is a tuple enum value where the first argument is
//! text left to parse, and second is retrieved AST value.
//! `IResult::Error` means that parse did fail.

use ast::{BinaryOperator, Expression, Statement, VariableName};

use std::str::{self, FromStr};

use nom;
pub use nom::IResult;
use unicode_xid::UnicodeXID;

fn valid_identifier_first_character(result: &str) -> bool {
    let message = "take_s did not take one parameter";
    let result = result.chars().next().expect(message);
    result == '!' || result == '_' || UnicodeXID::is_xid_start(result)
}

fn valid_later_character(c: char) -> bool {
    UnicodeXID::is_xid_continue(c) || c == '_'
}

named!(
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
///     use mvp_parser::parser::{self, IResult};
///
///     let parsed = parser::identifier("世界");
///     assert_eq!(parsed, IResult::Done("", String::from("世界")));
,
pub identifier<&str, String>, do_parse!(
    first: verify!(take_s!(1), valid_identifier_first_character) >>
    res: take_while_s!(valid_later_character) >>
    (format!("{}{}", first, res))
));

named!(
/// Assignment statement parser.
///
/// It expects variable name, followed by `=` character, and an expression
/// which marks expression to be stored as value.
///
/// # Examples
///
///     use mvp_parser::parser::{self, IResult};
///     use mvp_parser::ast::{Expression, Statement, VariableName};
///
///     let parsed = parser::assignment("hello = 44");
///     let expected = Statement::Assignment(
///         VariableName(String::from("hello")),
///         Expression::Number { value: 44, width: None },
///     );
///     assert_eq!(parsed, IResult::Done("", expected))
,
pub assignment<&str, Statement>, ws!(do_parse!(
    name: identifier >>
    tag!("=") >>
    value: expression >>
    (Statement::Assignment(VariableName(name), value))
)));

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
///     use mvp_parser::parser::{self, IResult};
///     use mvp_parser::ast::{BinaryOperator, Expression};
///
///     let parsed = parser::expression("2 + 3");
///     let expected = IResult::Done("", Expression::Binary(
///         BinaryOperator::Add,
///         Box::new(Expression::Number { value: 2, width: None }),
///         Box::new(Expression::Number { value: 3, width: None }),
///     ));
///     assert_eq!(parsed, expected);
,
pub expression<&str, Expression>, do_parse!(
    init: term >>
    res: fold_many0!(
        pair!(alt!(
            tag!("+") => {|_| BinaryOperator::Add}
            | tag!("-") => {|_| BinaryOperator::Sub}
        ), term),
        init,
        |first, (operator, another)| {
            Expression::Binary(operator, Box::new(first), Box::new(another))
        }
    ) >>
    (res)
));

named!(term<&str, Expression>, do_parse!(
    init: top_expression >>
    res: fold_many0!(
        pair!(alt!(
            tag!("*") => {|_| BinaryOperator::Mul}
            | tag!("/") => {|_| BinaryOperator::Div}
        ), top_expression),
        init,
        |first, (operator, another)| {
            Expression::Binary(operator, Box::new(first), Box::new(another))
        }
    ) >>
    (res)
));

named!(top_expression<&str, Expression>, alt!(paren_expression | number | hex_number | call));

named!(paren_expression<&str, Expression>, ws!(delimited!(tag!("("), expression, tag!(")"))));

named!(number<&str, Expression>, map!(
    map_res!(
        ws!(nom::digit),
        u32::from_str
    ),
    |value| Expression::Number { value: value, width: None }
));

named!(hex_number<&str, Expression>, ws!(do_parse!(
    tag!("$") >>
    number: map!(
        map_res!(nom::hex_digit, |s| u32::from_str_radix(s, 16).map(|value| (s.len(), value))),
        |(width, value)| Expression::Number { value: value, width: Some(width) }
    ) >>
    (number)
)));

named!(call<&str, Expression>, ws!(do_parse!(
    identifier: identifier >>
    parts: delimited!(
        tag!("("),
        separated_list!(tag!(","), expression),
        tag!(")")
    ) >>
    (Expression::Call(VariableName(identifier), parts))
)));