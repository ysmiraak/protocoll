#![feature(test)]

extern crate protocoll;
extern crate test;

use test::Bencher;
use protocoll::Map;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::Add;

#[bench]
fn imperative(b: &mut Bencher) {
    let sent = "a short treatise on fungi";
    let m = sent.chars().fold(HashMap::new(), |m,c| m.update_in_place(c, 0, |n| *n += 1));
    
    b.iter(|| {
        let mut m = m.clone();
        for (c,n) in m.clone() {
            match m.entry(c) {
                Entry::Occupied(e) => {*e.into_mut() += n},
                Entry::Vacant(e) => {e.insert(n);}
            }
        }
        m
    })
}

#[bench]
fn functional(b: &mut Bencher) {
    let sent = "a short treatise on fungi";
    let m = sent.chars().fold(HashMap::new(), |m,c| m.update_in_place(c, 0, |n| *n += 1));

    b.iter(|| Map::merge(m.clone(), m.clone(), usize::add))
}

#[bench]
fn in_place(b: &mut Bencher) {
    let sent = "a short treatise on fungi";
    let m =sent.chars().fold(HashMap::new(), |m,c| m.update_in_place(c, 0, |n| *n += 1));

    b.iter(|| Map::merge_in_place(m.clone(), m.clone(), |u,v| *u += v))
}
