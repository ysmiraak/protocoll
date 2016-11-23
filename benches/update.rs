#![feature(test)]

extern crate protocoll;
extern crate test;

use test::Bencher;
use protocoll::Map;
use std::collections::HashMap;

#[bench]
fn imperative(b: &mut Bencher) {
    let sent = "a short treatise on fungi";
    
    b.iter(|| {
        let mut letters = HashMap::new();

        for ch in sent.chars() {
            let counter = letters.entry(ch).or_insert(0);
            *counter += 1;
        }
        letters
    })
}

#[bench]
fn functional(b: &mut Bencher) {
    let sent = "a short treatise on fungi";

    b.iter(|| sent.chars().fold(HashMap::new(), |m,c| m.update(c, |n| 1 + n.unwrap_or(0))))
}

#[bench]
fn in_place(b: &mut Bencher) {
    let sent = "a short treatise on fungi";

    b.iter(|| sent.chars().fold(HashMap::new(), |m,c| m.update_in_place(c, 0, |n| *n += 1)))
}
