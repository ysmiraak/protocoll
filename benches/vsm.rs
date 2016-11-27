#![feature(test)]

extern crate protocoll;
extern crate test;

use test::Bencher;
use protocoll::Map;
use protocoll::map::VecSortedMap;

#[bench]
fn bench_update(b: &mut Bencher) {
    let sent = "a short treatise on fungi";

    b.iter(|| sent.chars().fold(VecSortedMap::new(), |m,c| m.update(c, |n| 1 + n.unwrap_or(0))))
}

#[bench]
fn bench_update_in_place(b: &mut Bencher) {
    let sent = "a short treatise on fungi";

    b.iter(|| sent.chars().fold(VecSortedMap::new(), |m,c| m.update_in_place(c, 0, |n| *n += 1)))
}
