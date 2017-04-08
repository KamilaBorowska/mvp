#![feature(test)]

extern crate mvp;
extern crate test;

use mvp::parser::grammar;
use test::Bencher;

#[bench]
fn address(b: &mut Bencher) {
    b.iter(|| grammar::statement("LDA $19"));
}
