use std::collections::{HashMap,BTreeMap};
use std::borrow::Borrow;
use std::hash::Hash;

/// basic protocol for maps.
pub trait Map<K,V> where Self:Sized {
    /// a map maps from keys to values.
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Ord;

    /// adds entry `kv`.
    fn inc(self, kv:(K,V)) -> Self;

    /// removes key `k`.
    fn dec<Q:?Sized>(self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Ord;

    /// like clojure's [update](http://clojuredocs.org/clojure.core/update).
    fn update<F>(self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V ;

    /// like clojure's [merge-with](http://clojuredocs.org/clojure.core/merge-with).
    fn merge<M,F>(self, other:M, mut f:F) -> Self where M:IntoIterator<Item = (K,V)>, F:FnMut(V,V) -> V
    { other.into_iter().fold(self, |m,(k,v)| Map::update(m, k, |mu| match mu { Some(u) => f(u,v), None => v }))}

    /// takes another collection into this one.
    fn absorb<I>(self, coll:I) -> Self where I:IntoIterator<Item = (K,V)>
    { coll.into_iter().fold(self, Map::inc)}

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;

    /// `clear`.
    fn empty(self) -> Self;
}

impl<K,V> Map<K,V> for HashMap<K,V> where K:Hash+Eq {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Eq
    { Box::new(move |k| self.get(k))}

    fn inc(mut self, kv:(K,V)) -> Self
    { self.insert(kv.0, kv.1); self }

    fn dec<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Eq
    { self.remove(k); self }

    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    { let v = f(self.remove(&k)); Map::inc(self,(k,v))}

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(mut self) -> Self
    { self.clear(); self }
}

impl<K,V> Map<K,V> for BTreeMap<K,V> where K:Ord {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Ord
    { Box::new(move |k| self.get(k))}

    fn inc(mut self, kv:(K,V)) -> Self
    { self.insert(kv.0, kv.1); self }

    fn dec<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Ord
    { self.remove(k); self }

    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    { let v = f(self.remove(&k)); Map::inc(self,(k,v))}

    fn shrink(self) -> Self
    { self }

    fn empty(mut self) -> Self
    { self.clear(); self }
}
