use coll::Coll;
use std::collections::{VecDeque,BinaryHeap};

/// basic protocol for seqs.
pub trait Seq<T> where Self:Sized+Coll<T> {
    /// a seq maps from indices to items. O(n) for `BinaryHeap`.
    fn seqfn<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>;
    /// removes an item. for `Vec` it's the last one; for `VecDeque` the first;
    /// for `BinaryHeap` it's the greatest one. see also `Coll::inc`.
    fn dec(self) -> Self;
}

impl<T> Seq<T> for Vec<T> {
    fn seqfn<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    { Box::new(move |i| self.get(i))}
    fn dec(mut self) -> Self { self.pop(); self }
}

impl<T> Seq<T> for VecDeque<T> {
    fn seqfn<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    { Box::new(move |i| self.get(i))}
    fn dec(mut self) -> Self { self.pop_front(); self }
}

impl<T> Seq<T> for BinaryHeap<T> where T:Ord {
    fn seqfn<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    { Box::new(move |i| self.iter().skip(i).next())}
    fn dec(mut self) -> Self { self.pop(); self }
}


// fn main() {
//     let v:Vec<_> = (0..9).collect();
//     let d:VecDeque<_> = (0..9).collect();
//     let b:BinaryHeap<_> = (0..9).collect();

//     println!("{:?}",v.seqfn()(4));
//     println!("{:?}",d.seqfn()(4));
//     println!("{:?}",b.seqfn()(4));

//     println!("{:?}",v.dec());
//     println!("{:?}",d.dec());
//     println!("{:?}",b.dec());
// }
