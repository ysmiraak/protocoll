use _map::{Map,MapMut};
use std::borrow::Borrow;
use std::slice::{Iter};
use std::vec::IntoIter;
use std::iter::FromIterator;
use std::ops::Index;
use std::fmt::{Debug,Formatter,Result};

/// an array-map sorted by key. very efficient for small maps.
///
/// for explanations about the methods, see
/// [`BTreeMap`](https://doc.rust-lang.org/nightly/std/collections/struct.BTreeMap.html)
/// and [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
///
/// does not support `entry`; see [`Map::update`](#method.update) for the same
/// functionality. [`MapMut`](../trait.MapMut.html) functions are **not** much more
/// efficient than [`Map`](../trait.Map.html) for this data structure.
#[derive(Default,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct VecSortedMap<K,V>(Vec<(K,V)>);

impl<K,V> VecSortedMap<K,V> where K:Ord {
    pub fn new() -> Self
    {VecSortedMap(Vec::new())}

    pub fn with_capacity(n:usize) -> Self
    {VecSortedMap(Vec::with_capacity(n))}

    pub fn capacity(&self) -> usize
    {self.0.capacity()}

    pub fn reserve(&mut self, n:usize)
    {self.0.reserve(n)}

    pub fn shrink_to_fit(&mut self)
    {self.0.shrink_to_fit()}

    pub fn clear(&mut self)
    {self.0.clear()}

    /// O(log(len))
    pub fn contains_key<Q:?Sized>(&self, k:&Q) -> bool where K:Borrow<Q>, Q:Ord
    {match self.get(k) {Some(_) => true, None => false}}

    /// O(log(len))
    pub fn get<Q:?Sized>(&self, k:&Q) -> Option<&V> where K:Borrow<Q>, Q:Ord
    {match self.0.binary_search_by(|&(ref q, _)| q.borrow().cmp(&k))
     {Ok(i) => Some(&self.0[i].1),
      Err(_) => None}}

    /// O(log(len))
    pub fn get_mut<Q:?Sized>(&mut self, k:&Q) -> Option<&mut V> where K:Borrow<Q>, Q:Ord
    {match self.0.binary_search_by(|&(ref q, _)| q.borrow().cmp(&k))
     {Ok(i) => Some(&mut self.0[i].1),
      Err(_) => None}}

    /// O(log(len)) when `k` already exists. O(len) for inserting a new entry,
    /// caused by shifting all entries after it, which can be avoided by always
    /// inserting in order.
    pub fn insert(&mut self, k:K, v:V) -> Option<V>
    {let ref mut vec = self.0;
     match vec.binary_search_by(|&(ref q, _)| q.cmp(&k))
     {Ok(i) => {vec.push((k,v)); Some(vec.swap_remove(i).1)}
      Err(i) => {vec.insert(i,(k,v)); None}}}

    /// O(log(len)) when `k` does not exist. O(len) for removing an entry,
    /// because of the need for shifting all entries after it.
    pub fn remove<Q:?Sized>(&mut self, k:&Q) -> Option<V> where K:Borrow<Q>, Q:Ord
    {match self.0.binary_search_by(|&(ref q, _)| q.borrow().cmp(&k))
     {Ok(i) => Some(self.0.remove(i).1),
      Err(_) => None}}

    pub fn append(&mut self, other:&mut VecSortedMap<K,V>)
    {self.0.append(&mut other.0)}

    pub fn reserve_exact(&mut self, additional:usize)
    {self.0.reserve_exact(additional)}

    pub fn pop(&mut self) -> Option<(K,V)>
    {self.0.pop()}

    pub fn truncate(&mut self, len:usize)
    {self.0.truncate(len)}

    pub fn retain<F>(&mut self, f:F) where F:FnMut(&(K,V)) -> bool
    {self.0.retain(f)}

    pub fn split_off(&mut self, at:usize) -> VecSortedMap<K,V>
    {VecSortedMap(self.0.split_off(at))}
}

impl<K,V> VecSortedMap<K,V> {
    /// a view for the underlying vec. `&self` methods for `Vec` such as `get`
    /// and `split` can be accessed through this.
    pub fn view_content<'a>(&'a self) -> &'a [(K,V)]
    {&self.0}

    /// iterate over the underlying vec. note: iterator element type is **not**
    /// `(&K,&V)` but rather `&(K,V)`. `iter_mut` is not supported for this
    /// collection. see [`update_all_mut`](#method.update_all_mut) for
    /// the same functionality.
    pub fn iter(&self) -> Iter<(K,V)>
    {self.0.iter()}

    pub fn len(&self) -> usize
    {self.0.len()}

    pub fn is_empty(&self) -> bool
    {self.0.is_empty()}
}

impl<K,V> IntoIterator for VecSortedMap<K,V>
{type Item = (K,V); type IntoIter = IntoIter<(K,V)>;
 fn into_iter(self) -> IntoIter<(K,V)> {self.0.into_iter()}}

impl<'a,K:'a,V:'a> IntoIterator for &'a VecSortedMap<K,V>
{type Item = &'a (K,V); type IntoIter = Iter<'a,(K,V)>;
 fn into_iter(self) -> Iter<'a,(K,V)> {self.iter()}}

impl<K,V> Extend<(K,V)> for VecSortedMap<K,V> where K:Ord
{fn extend<I>(&mut self, iter:I) where I:IntoIterator<Item = (K,V)>
 {for (k,v) in iter {self.insert(k,v);}}}

impl<'a,K,V> Extend<(&'a K, &'a V)> for VecSortedMap<K,V> where K:Ord+Copy, V:Copy
{fn extend<I>(&mut self, iter:I) where I:IntoIterator<Item = (&'a K, &'a V)>
 {self.extend(iter.into_iter().map(|(&key,&value)| (key,value)));}}

impl<K,V> FromIterator<(K,V)> for VecSortedMap<K,V> where K:Ord
{fn from_iter<I>(iter:I) -> VecSortedMap<K,V> where I:IntoIterator<Item = (K,V)>
 {Map::plus(VecSortedMap::new(),iter)}}

impl<'a,K,Q:?Sized,V> Index<&'a Q> for VecSortedMap<K,V> where K:Ord, K:Borrow<Q>, Q:Ord
{type Output = V; fn index(&self, k:&Q) -> &V {self.get(k).expect("no entry found for key")}}

impl<K,V> Debug for VecSortedMap<K,V> where K:Ord+Debug, V:Debug
{fn fmt(&self, fmt: &mut Formatter) -> Result
 {fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k,v))).finish()}}

impl<K,V> Map<K,V> for VecSortedMap<K,V> where K:Ord {
    fn fun<'a,Q:?Sized>(&'a self) -> Box<Fn(&Q) -> Option<&'a V> + 'a> where K:Borrow<Q>, Q:Ord
    {Box::new(move |k| self.get(k))}

    fn inc(mut self, k:K, v:V) -> Self
    {self.insert(k,v); self}

    fn dec<Q:?Sized>(mut self, k:&Q) -> Self where K:Borrow<Q>, Q:Ord
    {self.remove(k); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}

    fn update<F>(mut self, k:K, f:F) -> Self where F:FnOnce(Option<V>) -> V
    {match self.0.binary_search_by(|&(ref q, _)| q.borrow().cmp(&k))
     {Err(i) => {self.0.insert(i,(k,f(None)))}
      Ok(i) => {let ref mut vec = self.0;
                let (_,v) = vec.swap_remove(i);
                vec.push((k,f(Some(v))));
                let l = vec.len();
                vec.swap(i,l-1)}} self}

    fn update_all<F>(self, mut f:F) -> Self
        where Self:IntoIterator<Item = (K,V)> + FromIterator<(K, V)>, F:FnMut(&K,V) -> V
    {VecSortedMap(self.0.into_iter().map(|(k,v)| {let v = f(&k,v); (k,v)}).collect())}
}

impl<K,V> MapMut<K,V> for VecSortedMap<K,V> where K:Ord {
    fn update_mut<F>(&mut self, k:K, mut fnil:V, f:F) where F:FnOnce(&mut V)
    {match self.0.binary_search_by(|&(ref q, _)| q.borrow().cmp(&k))
     {Ok(i) => f(&mut self.0[i].1), Err(i) => {f(&mut fnil); self.0.insert(i,(k,fnil))}}}

    /// this makes up for the (intended) absence of `iter_mut`.
    ///
    /// # example
    /// ```
    /// // a somewhat unecessary way to create a mapping from square numbers to
    /// // fibonacci numbers.
    /// use protocoll::map::VecSortedMap;
    /// use protocoll::MapMut;
    /// let mut m:VecSortedMap<u32,u32> =
    ///     (0..).map(|n| (n * n, 0))
    ///     .take(13).collect();
    /// let (ref mut a, ref mut b) = (0,1);
    /// m.update_all_mut
    ///     (|_,v| {*v = *a;
    ///             *a = *b;
    ///             *b += *v});
    /// assert_eq!(m[&   0], 0);
    /// assert_eq!(m[&   1], 1);
    /// assert_eq!(m[&   4], 1);
    /// assert_eq!(m[&   9], 2);
    /// assert_eq!(m[&  16], 3);
    /// assert_eq!(m[&  25], 5);
    /// assert_eq!(m[&  36], 8);
    /// assert_eq!(m[& 144], 144);
    /// ```
    fn update_all_mut<F>(&mut self, mut f:F) where F:FnMut(&K, &mut V)
    {for &mut (ref k, ref mut v) in &mut self.0 {f(k,v)}}

    fn merge_mut<I,F>(&mut self, coll:I, mut f:F) where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V)
    {for (k,v) in coll
     {match self.0.binary_search_by(|&(ref q, _)| q.cmp(&k))
      {Ok(i) => f(&mut self.0[i].1, v),
       Err(i) => self.0.insert(i,(k,v))}}}
}
