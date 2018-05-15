extern crate mvp;

use mvp::parser::grammar::{self, CompleteStr};

macro_rules! test {
    ($name:ident, $input:expr) => {
        test!($name, $input, "", $input);
    };

    ($name:ident, $input:expr, $unparsed:expr, $output:expr) => {
        #[test]
        fn $name() {
            let parsed = grammar::identifier(CompleteStr($input));
            assert_eq!(parsed, Ok((CompleteStr($unparsed), $output)));
        }
    };
}

test!(simple_identifier, "hello");
test!(numbers_at_end, "abc123");
test!(partial_parse, "he+", "+", "he");
test!(exclamation, "!variable");
test!(single_exclamation, "!");
test!(exclamation_only_at_beginning, "!!", "!", "!");
test!(underscore, "a___b");
test!(underscore_at_beginning, "_private");
test!(unicode, "世界");
test!(unicode_continue, "a﹏b");

#[test]
fn parse_failure() {
    let parsed = grammar::identifier(CompleteStr("4"));
    assert!(parsed.is_err());
}

#[test]
fn unicode_continue_cannot_be_at_beginning() {
    let parsed = grammar::identifier(CompleteStr("﹏"));
    assert!(parsed.is_err());
}
