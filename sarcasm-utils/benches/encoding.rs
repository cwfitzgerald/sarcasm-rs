#![feature(test)]
extern crate test;

use sarcasm_utils::{encode_sarcasm, StartingCase};

#[bench]
fn encoding_ascii(b: &mut test::Bencher) {
    let string: String = (b'a'..=b'z')
        .map(|c| c as char)
        .chain("\n".chars())
        .cycle()
        .take(1_000_000)
        .collect();
    b.iter(|| {
        encode_sarcasm(&string, StartingCase::Uppercase);
    })
}
