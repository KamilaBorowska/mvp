#![feature(test)]

extern crate mvp;
extern crate test;

use mvp::parser::grammar::{self, CompleteStr};
use test::Bencher;

#[bench]
fn identifier(b: &mut Bencher) {
    b.iter(|| grammar::identifier(CompleteStr("LDA")));
}

#[bench]
fn address(b: &mut Bencher) {
    b.iter(|| grammar::statement(CompleteStr("LDA $19")));
}

#[bench]
fn address_ambiguous_parse(b: &mut Bencher) {
    b.iter(|| grammar::statement(CompleteStr("LDA ($19)+2")));
}
