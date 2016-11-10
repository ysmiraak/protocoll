use std::hash::Hash;
use std::borrow::Borrow;
use std::collections::{HashSet,BTreeSet};

/// basic protocol for sets.
pub trait Set<T> where Self:Sized {
    /// a set maps from items to themselves.
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Hash+Ord;

    /// adds item `i`.
    fn inc(self, i:T) -> Self;

    /// removes item `i`.
    fn dec<Q:?Sized>(self, i:&Q) -> Self where T:Borrow<Q>, Q:Hash+Ord;

    /// takes another collection into this one.
    fn absorb<I>(self, coll:I) -> Self where I:IntoIterator<Item = T>
    { coll.into_iter().fold(self, Set::inc)}

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;

    /// `clear`.
    fn empty(self) -> Self;
}

impl<T> Set<T> for HashSet<T> where T:Hash+Eq {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Hash+Eq
    { Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    { self.insert(i); self }

    fn dec<Q:?Sized>(mut self, i:&Q) -> Self where T:Borrow<Q>, Q:Hash+Eq
    { self.remove(i); self }

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(mut self) -> Self
    { self.clear(); self }
}

impl<T> Set<T> for BTreeSet<T> where T:Ord {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Ord
    { Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    { self.insert(i); self }

    fn dec<Q:?Sized>(mut self, i:&Q) -> Self where T:Borrow<Q>, Q:Ord
    { self.remove(i); self }

    fn shrink(self) -> Self
    { self }

    fn empty(mut self) -> Self
    { self.clear(); self }
}


// fn main() {
//     let s1 = (0..6).fold(HashSet::new(), Set::inc);
//     let s2 = (3..9).fold(HashSet::new(), Set::inc);

//     println!("{:?}",s1);
//     println!("{:?}",s2);

//     let s1 = Set::dec(s1,&0);
//     println!("{:?}",s1);

//     let s2f = s2.fun();

//     for i in &s1 { println!("{:?}",s2f(i))}

//     for i in s1.into_iter().filter_map(|i|s2f(&i)) { println!("{:?}",i)}
// }
