use std::collections::{HashMap,HashSet,BTreeMap,BTreeSet,BinaryHeap,VecDeque};
use std::hash::Hash;

/// basic protocol for collections.
pub trait Coll<T> {
    /// increment to incorporate `i`. both `Vec` and `VecDeque` grows to the right.
    fn inc(self, i:T) -> Self;
    /// pour another `coll` into this one.
    fn into<I>(self, coll:I) -> Self where Self:Sized, I:IntoIterator<Item = T>
    { coll.into_iter().fold(self, Coll::inc)}
    /// `shrink_to_fit`.
    fn shrink(self) -> Self;
    /// `clear`.
    fn empty(self) -> Self;
}

impl<K,V> Coll<(K,V)> for HashMap<K,V> where K:Hash+Eq {
    fn inc(mut self, i:(K,V)) -> Self { self.insert(i.0, i.1); self }
    fn shrink(mut self) -> Self { self.shrink_to_fit(); self }
    fn empty(mut self) -> Self { self.clear(); self }
}

impl<K,V> Coll<(K,V)> for BTreeMap<K,V> where K:Ord {
    fn inc(mut self, i:(K,V)) -> Self { self.insert(i.0, i.1); self }
    fn shrink(self) -> Self { self }
    fn empty(mut self) -> Self { self.clear(); self }
}

impl<T> Coll<T> for HashSet<T> where T:Hash+Eq {
    fn inc(mut self, i:T) -> Self { self.insert(i); self }
    fn shrink(mut self) -> Self { self.shrink_to_fit(); self }
    fn empty(mut self) -> Self { self.clear(); self }
}

impl<T> Coll<T> for BTreeSet<T> where T:Ord {
    fn inc(mut self, i:T) -> Self { self.insert(i); self }
    fn shrink(self) -> Self { self }
    fn empty(mut self) -> Self { self.clear(); self }
}

impl<T> Coll<T> for BinaryHeap<T> where T:Ord {
    fn inc(mut self, i:T) -> Self { self.push(i); self }
    fn shrink(mut self) -> Self { self.shrink_to_fit(); self }
    fn empty(mut self) -> Self { self.clear(); self }    
}

impl<T> Coll<T> for Vec<T> {
    fn inc(mut self, i:T) -> Self { self.push(i); self }
    fn shrink(mut self) -> Self { self.shrink_to_fit(); self }
    fn empty(mut self) -> Self { self.clear(); self }    
}

impl<T> Coll<T> for VecDeque<T> {
    fn inc(mut self, i:T) -> Self { self.push_back(i); self }
    fn shrink(mut self) -> Self { self.shrink_to_fit(); self }
    fn empty(mut self) -> Self { self.clear(); self }    
}

// TODO consider implementing:
// http://clojuredocs.org/clojure_core/clojure.core/frequencies
// http://clojuredocs.org/clojure.core/group-by
// http://clojuredocs.org/clojure.core/reduce-kv
// http://clojuredocs.org/clojure.set/index
