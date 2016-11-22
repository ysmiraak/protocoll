use std::collections::{HashMap,hash_map,BTreeMap,btree_map};
use std::borrow::Borrow;
use std::hash::Hash;

/// basic protocol for maps.
pub trait Map<K,V> where Self:Sized {
    /// a map maps from keys to values.
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Hash+Ord;

    /// adds `v` at `k`.
    fn inc(self, k:K, v:V) -> Self;

    /// removes key `k`.
    fn dec<Q:?Sized>(self, k:&Q) -> Self where K:Borrow<Q>, Q:Hash+Ord;

    /// pours another collection into this one.
    fn plus<I>(self, coll:I) -> Self where I:IntoIterator<Item = (K,V)>
    {coll.into_iter().fold(self, |m,(k,v)| Map::inc(m,k,v))}

    /// `clear`.
    fn zero(self) -> Self;

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;

    /// like clojure's [update](http://clojuredocs.org/clojure.core/update).
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
    fn update<F>(self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V ;

    /// like clojure's [merge-with](http://clojuredocs.org/clojure.core/merge-with).
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
    {coll.into_iter().fold(self, |m,(k,v)| Map::update(m, k, |opt_u| match opt_u {Some(u) => f(u,v), None => v }))}

    /// like `Map::update` but more efficient.
    /// # example
    /// ```
    /// use protocoll::Map;
    /// use std::collections::HashMap;
    /// let a = [0,0,0,1,1,0,0,0];
    /// let m1 = a.iter().fold
    ///     (HashMap::new(), |m,&k| Map::update
    ///      (m, k, |n| 1 + n.unwrap_or(0)));
    /// let m2 = a.iter().fold
    ///     (HashMap::new(), |m,&k| Map::update_in_place
    ///      (m, k, 0, |n| *n += 1));
    /// assert_eq!(m1,m2);
    /// ```
    fn update_in_place<F>(self, k:K, fnil:V, f:F) -> Self where F:FnOnce(&mut V);

    /// like `Map::merge` but more efficient.
    /// # example
    /// ```
    /// use protocoll::Map;
    /// use std::collections::HashMap;
    /// use std::ops::Add;
    /// let a = [0,0,0,1,1,0,0,0];
    /// let m = a.iter().fold
    ///     (HashMap::new(), |m,&k| Map::update_in_place
    ///      (m, k, 0, |n| *n += 1));
    /// let m = Map::merge_in_place(m.clone(), m, |u,v| *u += v);
    /// assert_eq!(12, m[&0]);
    /// assert_eq!(4, m[&1]);
    /// ```
    fn merge_in_place<I,F>(self, coll:I, f:F) -> Self where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V);
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

    fn update_in_place<F>(mut self, k:K, fnil:V, f:F) -> Self where F:FnOnce(&mut V)
    {f(self.entry(k).or_insert(fnil)); self}

    fn merge_in_place<I,F>(self, coll:I, mut f:F) -> Self where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V)
    {coll.into_iter().fold
     (self, |mut m,(k,v)|
      {match m.entry(k)
       {hash_map::Entry::Occupied(e) => f(e.into_mut(),v),
        hash_map::Entry::Vacant(e) => {e.insert(v);}} m})}

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

    fn update_in_place<F>(mut self, k:K, fnil:V, f:F) -> Self where F:FnOnce(&mut V)
    {f(self.entry(k).or_insert(fnil)); self}

    fn merge_in_place<I,F>(self, coll:I, mut f:F) -> Self where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V)
    {coll.into_iter().fold
     (self, |mut m,(k,v)|
      {match m.entry(k)
       {btree_map::Entry::Occupied(e) => f(e.into_mut(),v),
        btree_map::Entry::Vacant(e) => {e.insert(v);}} m})}
}
