use coll::Coll;
use std::hash::Hash;
use std::borrow::Borrow;
use std::collections::{HashMap,BTreeMap};

/// basic protocol for maps.
pub trait Map<K,V> where Self:Sized+Coll<(K,V)> {
    /// a map maps from keys to values.
    fn mapfn<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Eq+Ord;
    /// like clojure's [assoc](http://clojuredocs.org/clojure.core/assoc).
    fn assoc(self, k:K, v:V) -> Self { Coll::inc(self,(k,v)) }
    /// like clojure's [dissoc](http://clojuredocs.org/clojure.core/dissoc).
    fn dissoc<Q:?Sized>(self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Eq+Ord;
    /// like clojure's [update](http://clojuredocs.org/clojure.core/update).
    fn update<F>(self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V ;
    /// like clojure's [merge-with](http://clojuredocs.org/clojure.core/merge-with).
    fn merge<M,F>(self, other:M, mut f:F) -> Self where M:IntoIterator<Item = (K,V)>, F:FnMut(V,V) -> V
    { other.into_iter().fold(self, |m,(k,v)| Map::update(m, k, |mu| match mu { Some(u) => f(u,v), None => v }))}
}

impl<K,V> Map<K,V> for HashMap<K,V> where K:Hash+Eq {
    fn mapfn<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Eq
    { Box::new(move |k| self.get(k))}
    fn dissoc<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Eq
    { self.remove(k); self }
    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    { let v = f(self.remove(&k)); Coll::inc(self,(k,v))}
}

impl<K,V> Map<K,V> for BTreeMap<K,V> where K:Ord {
    fn mapfn<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Ord
    { Box::new(move |k| self.get(k))}
    fn dissoc<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Ord
    { self.remove(k); self }
    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    { let v = f(self.remove(&k)); Coll::inc(self,(k,v))}
}

// fn main(){
//     let a1 = [1, 2, 3];
//     let a2 = [4, 5, 6];
//     let m1 = a1.iter().map(ToOwned::to_owned)
//         .zip(a2.iter().map(ToOwned::to_owned))
//         .fold(HashMap::new(), Coll::inc);
//     let m2 = a2.iter().map(ToOwned::to_owned)
//         .zip(a1.iter().map(ToOwned::to_owned))
//         .fold(HashMap::new(), Coll::inc);

//     println!("{:?}",m1);
//     println!("{:?}",m2);
//     let m1 = Map::update(m1, 0, &|_| 1);
//     println!("{:?}",m1);
//     println!("{:?}",m1.mapfn()(&0));
//     println!("{:?}",m1.mapfn()(&3));

//     println!("{:?}",m1.merge(m2,|_,_|0));
// }
