use std::collections::{HashMap,hash_map,BTreeMap,btree_map};
use std::borrow::Borrow;
use std::hash::Hash;
use std::iter::FromIterator;

/// basic protocol for maps.
pub trait Map<K,V> where Self:Sized {
    /// a map maps from keys to values.
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Ord;

    /// adds `v` at `k`.
    ///
    /// like `clojure`'s [`assoc`](http://clojuredocs.org/clojure.core/assoc).
    fn inc(self, k:K, v:V) -> Self;

    /// removes key `k`.
    ///
    /// like `clojure`'s [`dissoc`](http://clojuredocs.org/clojure.core/dissoc).
    fn dec<Q:?Sized>(self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Ord;

    /// pours another collection into this one.
    ///
    /// like `clojure`'s [`into`](http://clojuredocs.org/clojure.core/into).
    fn plus<I>(self, coll:I) -> Self where I:IntoIterator<Item = (K,V)>
    {coll.into_iter().fold(self, |m,(k,v)| Map::inc(m,k,v))}

    /// `clear`.
    fn zero(self) -> Self;

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;

    /// updates the value at `k` by `f`.
    ///
    /// like `clojure`'s [`update`](http://clojuredocs.org/clojure.core/update).
    ///
    /// # example
    /// ```
    /// use protocoll::Map;
    /// use std::collections::HashMap;
    /// let m = [0,0,0,1,1,0,0,0].iter().fold
    ///     (HashMap::new(), |m,&k| Map::update
    ///      (m, k, |n| 1 + n.unwrap_or(0)));
    /// assert_eq!(6, m[&0]);
    /// assert_eq!(2, m[&1]);
    /// ```
    fn update<F>(self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V;

    /// updates all values by `f`
    ///
    /// # example
    /// ```
    /// use protocoll::Map;
    /// use std::collections::HashMap;
    /// let m = [0,0,0,1,1,0,0,0].iter().fold
    ///     (HashMap::new(), |m,&k| Map::update
    ///      (m, k, |n| 1 + n.unwrap_or(0)))
    ///     .update_all(|_,v| v + 1);
    /// assert_eq!(7, m[&0]);
    /// assert_eq!(3, m[&1]);
    /// ```
    fn update_all<F>(self, mut f:F) -> Self
        where Self:IntoIterator<Item = (K,V)> + FromIterator<(K, V)>, F:FnMut(&K,V) -> V
    {self.into_iter().map(|(k,v)| {let v = f(&k,v); (k,v)}).collect()}

    /// merges `coll` into this one, resolving conflicts by `f`.
    ///
    /// like `clojure`'s [`merge-with`](http://clojuredocs.org/clojure.core/merge-with).
    ///
    /// # example
    /// ```
    /// use protocoll::Map;
    /// use std::collections::HashMap;
    /// use std::ops::Add;
    /// let m = [0,0,0,1,1,0,0,0].iter().fold
    ///     (HashMap::new(), |m,&k| Map::update
    ///      (m, k, |n| 1 + n.unwrap_or(0)));
    /// let m = Map::merge(m.clone(), m, usize::add);
    /// assert_eq!(12, m[&0]);
    /// assert_eq!(4, m[&1]);
    /// ```
    fn merge<I,F>(self, coll:I, mut f:F) -> Self where I:IntoIterator<Item = (K,V)>, F:FnMut(V,V) -> V
    {coll.into_iter().fold(self, |m,(k,v)| Map::update(m, k, |opt_u| match opt_u {Some(u) => f(u,v), None => v}))}
}

pub trait MapMut<K,V> {
    /// like [`Map::update`](trait.Map.html#tymethod.update) but can be more efficient.
    ///
    /// # example
    /// ```
    /// use protocoll::{Map,MapMut};
    /// use std::collections::HashMap;
    /// let a = [0,0,0,1,1,0,0,0];
    /// let m1 = a.iter().fold
    ///     (HashMap::new(), |m,&k| Map::update
    ///      (m, k, |n| 1 + n.unwrap_or(0)));
    /// let m2 = a.iter().fold
    ///     (HashMap::new(), |mut m, &k|
    ///      {m.update_mut(k, 0, |n| *n += 1); m});
    /// assert_eq!(m1,m2);
    /// ```
    fn update_mut<F>(&mut self, k:K, fnil:V, f:F) where F:FnOnce(&mut V);

    /// like [`Map::update_all`](trait.Map.html#method.update_all) but can be more efficient.
    ///
    /// # example
    /// ```
    /// use protocoll::MapMut;
    /// use std::collections::HashMap;
    /// let mut m = [0,0,0,1,1,0,0,0].iter().fold
    ///     (HashMap::new(), |mut m, &k|
    ///      {m.update_mut(k, 0, |n| *n += 1); m});
    /// m.update_all_mut(|_,v| *v += 1);
    /// assert_eq!(7, m[&0]);
    /// assert_eq!(3, m[&1]);
    /// ```
    fn update_all_mut<F>(&mut self, f:F) where F:FnMut(&K, &mut V);

    /// like [`Map::merge`](trait.Map.html#method.merge) but can be more efficient.
    ///
    /// # example
    /// ```
    /// use protocoll::MapMut;
    /// use std::collections::HashMap;
    /// let m1 = [0,0,0,1,1,0,0,0].iter().fold
    ///     (HashMap::new(), |mut m, &k|
    ///      {m.update_mut(k, 0, |n| *n += 1); m});
    /// let mut m2 = m1.clone();
    /// m2.merge_mut(m1, |u,v| *u += v);
    /// assert_eq!(12, m2[&0]);
    /// assert_eq!(4, m2[&1]);
    /// ```
    fn merge_mut<I,F>(&mut self, coll:I, f:F) where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V);
}

impl<K,V> Map<K,V> for HashMap<K,V> where K:Hash+Eq {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Eq
    {Box::new(move |k| self.get(k))}

    fn inc(mut self, k:K, v:V) -> Self
    {self.insert(k,v); self}

    fn dec<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Eq
    {self.remove(k); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}

    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    {let v = f(self.remove(&k)); Map::inc(self,k,v)}
}

impl<K,V> MapMut<K,V> for HashMap<K,V> where K:Hash+Eq {
    fn update_mut<F>(&mut self, k:K, fnil:V, f:F) where F:FnOnce(&mut V)
    {f(self.entry(k).or_insert(fnil))}

    fn update_all_mut<F>(&mut self, mut f:F) where F:FnMut(&K, &mut V)
    {for (k,v) in self {f(k,v)}}
    
    fn merge_mut<I,F>(&mut self, coll:I, mut f:F) where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V)
    {for (k,v) in coll
     {match self.entry(k)
      {hash_map::Entry::Occupied(e) => f(e.into_mut(),v),
       hash_map::Entry::Vacant(e) => {e.insert(v);}}}}
}

impl<K,V> Map<K,V> for BTreeMap<K,V> where K:Ord {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Ord
    {Box::new(move |k| self.get(k))}

    fn inc(mut self, k:K, v:V) -> Self
    {self.insert(k,v); self}

    fn dec<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Ord
    {self.remove(k); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(self) -> Self
    {self}

    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    {let v = f(self.remove(&k)); Map::inc(self,k,v)}
}

impl<K,V> MapMut<K,V> for BTreeMap<K,V> where K:Ord {
    fn update_mut<F>(&mut self, k:K, fnil:V, f:F) where F:FnOnce(&mut V)
    {f(self.entry(k).or_insert(fnil))}

    fn update_all_mut<F>(&mut self, mut f:F) where F:FnMut(&K, &mut V)
    {for (k,v) in self {f(k,v)}}
    
    fn merge_mut<I,F>(&mut self, coll:I, mut f:F) where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V)
    {for (k,v) in coll
     {match self.entry(k)
      {btree_map::Entry::Occupied(e) => f(e.into_mut(),v),
       btree_map::Entry::Vacant(e) => {e.insert(v);}}}}
}
