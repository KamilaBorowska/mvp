extern crate mvp_parser;

use mvp_parser::parser::{self, IResult};

macro_rules! test {
    ($name:ident, $input:expr) => {
        test!($name, $input, "", $input);
    };

    ($name:ident, $input:expr, $unparsed:expr, $output:expr) => {
        #[test]
        fn $name() {
            let parsed = parser::identifier($input);
            assert_eq!(parsed, IResult::Done($unparsed, String::from($output)))
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

#[test]
fn parse_failure() {
    let parsed = parser::identifier("4");
    assert!(parsed.is_err());
}
