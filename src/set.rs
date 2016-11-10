use coll::Coll;
use std::hash::Hash;
use std::borrow::Borrow;
use std::collections::{HashSet,BTreeSet};

/// basic protocol for sets.
pub trait Set<T> where Self:Sized+Coll<T> {
    /// a set maps from items to themselves.
    fn setfn<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Hash+Eq+Ord;
    /// removes `i`.
    fn lose<Q: ?Sized>(self, i:&Q) -> Self where T:Borrow<Q>, Q:Hash+Eq+Ord;
}

impl<T> Set<T> for HashSet<T> where T:Hash+Eq {
    fn setfn<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Hash+Eq
    { Box::new(move |i| self.get(i))}
    fn lose<Q: ?Sized>(mut self, i:&Q) -> Self where T:Borrow<Q>, Q:Hash+Eq
    { self.remove(i); self }
}

impl<T> Set<T> for BTreeSet<T> where T:Ord {
    fn setfn<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Ord
    { Box::new(move |i| self.get(i))}
    fn lose<Q: ?Sized>(mut self, i:&Q) -> Self where T:Borrow<Q>, Q:Ord
    { self.remove(i); self }
}

// Vec???
// VecDeque???

// fn main(){
//     let s1 = (0..6).fold(HashSet::new(), Coll::inc);
//     let s2 = (3..9).fold(HashSet::new(), Coll::inc);

//     println!("{:?}",s1);
//     println!("{:?}",s2);

//     let s1 = Set::lose(s1,&0);
//     println!("{:?}",s1);

//     let s2f = s2.setfn();

//     for i in &s1 { println!("{:?}",s2f(i))}
    
//     for i in s1.into_iter().filter_map(|i|s2f(&i)) { println!("{:?}",i)}
// }
