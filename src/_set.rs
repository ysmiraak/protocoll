use std::collections::{HashSet,BTreeSet};
use std::borrow::Borrow;
use std::hash::Hash;

/// basic protocol for sets.
pub trait Set<T> where Self:Sized {
    /// a set maps from items to themselves.
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Hash+Ord;

    /// like `clojure`'s [`conj`](http://clojuredocs.org/clojure.core/conj)
    ///
    /// adds item `i`.
    fn inc(self, i:T) -> Self;

    /// like `clojure`'s [`disj`](http://clojuredocs.org/clojure.core/disj)
    ///
    /// removes item `i`.
    fn dec<Q:?Sized>(self, i:&Q) -> Self where T:Borrow<Q>, Q:Hash+Ord;

    /// like `clojure`'s [`into`](http://clojuredocs.org/clojure.core/into)
    ///
    /// pours another collection into this one.
    fn plus<I>(self, coll:I) -> Self where I:IntoIterator<Item = T>
    {coll.into_iter().fold(self, Set::inc)}

    /// `clear`.
    fn zero(self) -> Self;

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;
}

impl<T> Set<T> for HashSet<T> where T:Hash+Eq {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Hash+Eq
    {Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    {self.insert(i); self}

    fn dec<Q:?Sized>(mut self, i:&Q) -> Self where T:Borrow<Q>, Q:Hash+Eq
    {self.remove(i); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}
}

impl<T> Set<T> for BTreeSet<T> where T:Ord {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a T> + 'a> where T:Borrow<Q>, Q:Ord
    {Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    {self.insert(i); self}

    fn dec<Q:?Sized>(mut self, i:&Q) -> Self where T:Borrow<Q>, Q:Ord
    {self.remove(i); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(self) -> Self
    {self}
}
