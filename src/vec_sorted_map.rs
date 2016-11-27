use _map::Map;
use std::borrow::Borrow;

/// an array-map sorted by key. does not support `entry`; see
/// [`Map::update`](#method.update). [`Map::update_in_place`](#method.update_in_place)
/// is **not** much more efficient than `update` for this data structure.
#[derive(Default,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct VecSortedMap<K,V>(Vec<(K,V)>);

impl<K,V> VecSortedMap<K,V> where K:Ord {
    pub fn new() -> Self
    {VecSortedMap(Vec::new())}

    pub fn with_capacity(c:usize) -> Self
    {VecSortedMap(Vec::with_capacity(c))}

    pub fn capacity(&self) -> usize
    {self.0.capacity()}

    pub fn reserve(&mut self, c:usize)
    {self.0.reserve(c)}

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

    // todo consider porting in &mut methods from vec
    // fn pop(&mut self) -> Option<T>
    // fn truncate(&mut self, len: usize)
    // fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool
    // fn drain<R>(&mut self, range: R) -> Drain<T> where R: RangeArgument<usize>
    // fn split_off(&mut self, at: usize) -> Vec<T>
    // fn resize(&mut self, new_len: usize, value: T)
    // fn dedup(&mut self)
}

use std::slice::{Iter};
impl<K,V> VecSortedMap<K,V> {
    /// a view for the underlying vec. `&self` methods for `Vec` such as `get`
    /// and `split` can be accessed through this.
    pub fn view_content<'a>(&'a self) -> &'a Vec<(K,V)>
    {&self.0}

    /// iterate over the underlying vec. note: iterator element type is **not**
    /// `(&K,&V)` but rather `&(K,V)`. `iter_mut` is not supported for this
    /// collection. see [`update_all_in_place`](#method.update_all_in_place) for
    /// the same functionality.
    pub fn iter(&self) -> Iter<(K,V)>
    {self.0.iter()}

    pub fn len(&self) -> usize
    {self.0.len()}

    pub fn is_empty(&self) -> bool
    {self.0.is_empty()}

    /// this makes up for the (intentional) absence of `iter_mut`.
    /// # example
    /// ```    
    /// // a somewhat unecessary way to create a mapping from square numbers to
    /// // the fibonacci sequence.
    /// use protocoll::map::VecSortedMap;
    /// let m:VecSortedMap<u32,u32> =
    ///     (0..).map(|n| (n * n, 0))
    ///     .take(13).collect();
    /// let (ref mut a, ref mut b) = (0,1);
    /// let m = m.update_all_in_place
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
    pub fn update_all_in_place<F>(mut self, mut f:F) -> Self where F:FnMut(&K,&mut V)
    {for &mut (ref k, ref mut v) in &mut self.0 {f(k,v)} self}
}

use std::vec::IntoIter;
impl<K,V> IntoIterator for VecSortedMap<K,V>
{type Item = (K,V); type IntoIter = IntoIter<(K,V)>;
 fn into_iter(self) -> IntoIter<(K,V)> {self.0.into_iter()}}

impl<'a,K:'a,V:'a> IntoIterator for &'a VecSortedMap<K,V>
{type Item = &'a (K,V); type IntoIter = Iter<'a,(K,V)>;
 fn into_iter(self) -> Iter<'a,(K,V)> {self.iter()}}

impl<K,V> Extend<(K,V)> for VecSortedMap<K,V> where K:Ord
{fn extend<I>(&mut self, iter:I) where I:IntoIterator<Item = (K, V)>
 {for (k,v) in iter {self.insert(k,v);}}}

impl<'a,K,V> Extend<(&'a K, &'a V)> for VecSortedMap<K,V> where K:Ord+Copy, V:Copy
{fn extend<I>(&mut self, iter:I) where I:IntoIterator<Item = (&'a K, &'a V)>
 {self.extend(iter.into_iter().map(|(&key,&value)| (key,value)));}}

use std::iter::FromIterator;
impl<K,V> FromIterator<(K,V)> for VecSortedMap<K,V> where K:Ord
{fn from_iter<I>(iter:I) -> VecSortedMap<K,V> where I:IntoIterator<Item = (K, V)>
 {Map::plus(VecSortedMap::new(),iter)}}

use std::ops::Index;
impl<'a,K,Q:?Sized,V> Index<&'a Q> for VecSortedMap<K,V> where K:Ord, K:Borrow<Q>, Q:Ord
{type Output = V; fn index(&self, k:&Q) -> &V {self.get(k).expect("no entry found for key")}}

use std::fmt::{Debug,Formatter,Result};
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

    fn update_in_place<F>(mut self, k:K, mut fnil:V, f:F) -> Self where F:FnOnce(&mut V)
    {match self.0.binary_search_by(|&(ref q, _)| q.borrow().cmp(&k))
     {Ok(i) => f(&mut self.0[i].1), Err(i) => {f(&mut fnil); self.0.insert(i,(k,fnil))}} self}

    fn merge_in_place<I,F>(self, coll:I, mut f:F) -> Self where I:IntoIterator<Item = (K,V)>, F:FnMut(&mut V, V)
    {coll.into_iter().fold
     (self, |mut m,(k,v)|
      {match m.0.binary_search_by(|&(ref q, _)| q.cmp(&k))
       {Ok(i) => f(&mut m.0[i].1, v),
        Err(i) => m.0.insert(i,(k,v))} m})}
}
