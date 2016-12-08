use _set::Set;
use std::borrow::Borrow;
use std::slice::{Iter};
use std::vec::IntoIter;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::cmp::{min,max};
use std::cmp::Ordering::{self,Less,Equal,Greater};
use std::ops::{Index,BitOr,BitAnd,BitXor,Sub};
use std::fmt::{Debug,Formatter,Result};

/// an array-set. very efficient for small sets.
///
/// for explanations about the methods, see
/// [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
/// and [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
#[derive(Default,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct VecSortedSet<T>(Vec<T>);

impl<T> VecSortedSet<T> where T:Ord {
    pub fn new() -> Self
    {VecSortedSet(Vec::new())}

    pub fn with_capacity(n:usize) -> Self
    {VecSortedSet(Vec::with_capacity(n))}

    pub fn capacity(&self) -> usize
    {self.0.capacity()}

    pub fn reserve(&mut self, n:usize)
    {self.0.reserve(n)}

    pub fn shrink_to_fit(&mut self)
    {self.0.shrink_to_fit()}

    pub fn clear(&mut self)
    {self.0.clear()}

    /// O(log(len))
    pub fn contains<Q:?Sized>(&self, e:&Q) -> bool where T:Borrow<Q>, Q:Ord
    {match self.get(e) {Some(_) => true, None => false}}

    /// O(log(len))
    pub fn get<Q:?Sized>(&self, e:&Q) -> Option<&T> where T:Borrow<Q>, Q:Ord
    {match self.0.binary_search_by(|q| q.borrow().cmp(&e))
     {Ok(i) => Some(&self.0[i]), Err(_) => None}}

    /// O(log(len))
    pub fn get_mut<Q:?Sized>(&mut self, e:&Q) -> Option<&mut T> where T:Borrow<Q>, Q:Ord
    {match self.0.binary_search_by(|q| q.borrow().cmp(&e))
     {Ok(i) => Some(&mut self.0[i]), Err(_) => None}}

    /// O(log(len)) when `e` already exists. O(len) for inserting a new element,
    /// caused by shifting all elements after it, which can be avoided by always
    /// inserting in order.
    pub fn insert(&mut self, e:T) -> Option<T>
    {let ref mut vec = self.0;
     match vec.binary_search_by(|q| q.cmp(&e))
     {Ok(i) => {vec.push(e); Some(vec.swap_remove(i))}
      Err(i) => {vec.insert(i,e); None}}}

    /// O(log(len)) when `e` does not exist. O(len) for removing an element,
    /// because of the need for shifting all elements after it.
    pub fn remove<Q:?Sized>(&mut self, e:&Q) -> Option<T> where T:Borrow<Q>, Q:Ord
    {match self.0.binary_search_by(|q| q.borrow().cmp(&e))
     {Ok(i) => Some(self.0.remove(i)), Err(_) => None}}

    pub fn append(&mut self, other:&mut VecSortedSet<T>)
    {self.0.append(&mut other.0)}

    pub fn reserve_exact(&mut self, n:usize)
    {self.0.reserve_exact(n)}

    pub fn pop(&mut self) -> Option<T>
    {self.0.pop()}

    pub fn truncate(&mut self, n:usize)
    {self.0.truncate(n)}

    pub fn retain<F>(&mut self, f:F) where F:FnMut(&T) -> bool
    {self.0.retain(f)}

    pub fn split_off(&mut self, at:usize) -> VecSortedSet<T>
    {VecSortedSet(self.0.split_off(at))}

    pub fn is_disjoint(&self, other:&VecSortedSet<T>) -> bool
    {self.intersection(other).next().is_none()}

    pub fn is_subset(&self, other:&VecSortedSet<T>) -> bool
    {let mut this = self.iter(); let mut that = other.iter();
     let mut opt_x = this.next(); let mut opt_y = that.next();
     loop {match (opt_x,opt_y)
           {(None,_) => return true,
            (Some(_),None) => return false,
            (Some(x),Some(y)) => match x.cmp(y) {
                Less => return false,
                Equal => opt_x = this.next(),
                Greater => ()}}
           opt_y = that.next()}}
    
    pub fn is_superset(&self, other:&VecSortedSet<T>) -> bool
    {other.is_subset(self)}

    pub fn union<'a>(&'a self, other:&'a VecSortedSet<T>) -> Union<'a,T>
    {Union{this:self.iter().peekable(),that:other.iter().peekable()}}

    pub fn intersection<'a>(&'a self, other:&'a VecSortedSet<T>) -> Intersection<'a,T>
    {Intersection{this:self.iter().peekable(),that:other.iter().peekable()}}

    pub fn difference<'a>(&'a self, other:&'a VecSortedSet<T>) -> Difference<'a,T>
    {Difference{this:self.iter().peekable(),that:other.iter().peekable()}}

    pub fn symmetric_difference<'a>(&'a self, other:&'a VecSortedSet<T>) -> SymmetricDifference<'a,T>
    {SymmetricDifference{this:self.iter().peekable(),that:other.iter().peekable()}}
}

impl<T> VecSortedSet<T> {
    /// a view for the underlying vec. `&self` methods for `Vec` such as `get`
    /// and `split` can be accessed through this.
    pub fn view_content<'a>(&'a self) -> &'a Vec<T>
    {&self.0}

    /// iterate over the underlying vec.
    pub fn iter(&self) -> Iter<T>
    {self.0.iter()}

    pub fn len(&self) -> usize
    {self.0.len()}

    pub fn is_empty(&self) -> bool
    {self.0.is_empty()}
}

impl<T> IntoIterator for VecSortedSet<T>
{type Item = T; type IntoIter = IntoIter<T>;
 fn into_iter(self) -> IntoIter<T> {self.0.into_iter()}}

impl<'a,T:'a> IntoIterator for &'a VecSortedSet<T>
{type Item = &'a T; type IntoIter = Iter<'a,T>;
 fn into_iter(self) -> Iter<'a,T> {self.iter()}}

impl<T> Extend<T> for VecSortedSet<T> where T:Ord
{fn extend<I>(&mut self, iter:I) where I:IntoIterator<Item = T>
 {for e in iter {self.insert(e);}}}

impl<'a,T> Extend<(&'a T)> for VecSortedSet<T> where T:Ord+Copy
{fn extend<I>(&mut self, iter:I) where I:IntoIterator<Item = &'a T>
 {self.extend(iter.into_iter().map(|&e| e));}}

impl<T> FromIterator<T> for VecSortedSet<T> where T:Ord
{fn from_iter<I>(iter:I) -> VecSortedSet<T> where I:IntoIterator<Item = T>
 {Set::plus(VecSortedSet::new(),iter)}}

impl<'a,T,Q:?Sized> Index<&'a Q> for VecSortedSet<T> where T:Ord, T:Borrow<Q>, Q:Ord
{type Output = T; fn index(&self, k:&Q) -> &T {self.get(k).expect("no entry found for key")}}

impl<T> Debug for VecSortedSet<T> where T:Ord+Debug
{fn fmt(&self, fmt: &mut Formatter) -> Result
 {fmt.debug_set().entries(self.0.iter()).finish()}}

impl<T> Set<T> for VecSortedSet<T> where T:Ord {
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

impl<T> BitOr<VecSortedSet<T>> for VecSortedSet<T> where T:Ord {
    type Output = VecSortedSet<T>;
    /// union.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((s1 | s2).view_content(), &[1,2,3,4]);
    /// ```
    fn bitor(self, other:VecSortedSet<T>) -> VecSortedSet<T> {
        let mut vec = Vec::new();
        let mut s1 = self.into_iter();
        let mut s2 = other.into_iter();
        let mut opt_e1 = s1.next();
        let mut opt_e2 = s2.next();
        loop {
            let (e1,e2) = match (opt_e1,opt_e2) {
                (None,None) => break,
                (Some(e1),None) => {vec.push(e1); break}
                (None,Some(e2)) => {vec.push(e2); break}
                (Some(e1),Some(e2)) => (e1,e2)
            };
            match e1.cmp(&e2) {
                Less => {opt_e1 = s1.next(); opt_e2 = Some(e2); vec.push(e1)}
                Equal => {opt_e1 = s1.next(); opt_e2 = s2.next(); vec.push(e1)}
                Greater => {opt_e1 = Some(e1); opt_e2 = s2.next(); vec.push(e2)}
            }
        }
        for e in s1 {vec.push(e)}
        for e in s2 {vec.push(e)}
        VecSortedSet(vec)
    }
}

impl<'a,'b,T> BitOr<&'b VecSortedSet<T>> for &'a VecSortedSet<T> where T:Ord+Clone {
    type Output = VecSortedSet<T>;
    /// union with cloned members.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((&s1 | &s2).view_content(), &[1,2,3,4]);
    /// ```
    fn bitor(self, other:&VecSortedSet<T>) -> VecSortedSet<T>
    {self.union(other).cloned().collect()}
}

impl<T> BitAnd<VecSortedSet<T>> for VecSortedSet<T> where T:Ord {
    type Output = VecSortedSet<T>;
    /// intersection.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((s1 & s2).view_content(), &[2,3]);
    /// ```
    fn bitand(self, other:VecSortedSet<T>) -> VecSortedSet<T> {
        let mut vec = Vec::new();
        let mut s1 = self.into_iter();
        let mut s2 = other.into_iter();
        let mut opt_e1 = s1.next();
        let mut opt_e2 = s2.next();
        loop {
            let (e1,e2) = match (opt_e1,opt_e2) {
                (Some(e1),Some(e2)) => (e1,e2),
                _ => break
            };
            match e1.cmp(&e2) {
                Less => {opt_e1 = s1.next(); opt_e2 = Some(e2)}
                Equal => {opt_e1 = s1.next(); opt_e2 = s2.next(); vec.push(e1)}
                Greater => {opt_e1 = Some(e1); opt_e2 = s2.next()}
            }
        }
        VecSortedSet(vec)
    }
}

impl<'a,'b,T> BitAnd<&'b VecSortedSet<T>> for &'a VecSortedSet<T> where T:Ord+Clone {
    type Output = VecSortedSet<T>;
    /// intersection with cloned members.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((&s1 & &s2).view_content(), &[2,3]);
    /// ```
    fn bitand(self, other:&VecSortedSet<T>) -> VecSortedSet<T>
    {self.intersection(other).cloned().collect()}
}


impl<T> Sub<VecSortedSet<T>> for VecSortedSet<T> where T:Ord {
    type Output = VecSortedSet<T>;
    /// difference.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((s1 - s2).view_content(), &[1]);
    /// ```
    fn sub(self, other:VecSortedSet<T>) -> VecSortedSet<T> {
        let mut vec = Vec::new();
        let mut s1 = self.into_iter();
        let mut s2 = other.into_iter();
        let mut opt_e1 = s1.next();
        let mut opt_e2 = s2.next();
        loop {
            let (e1,e2) = match (opt_e1,opt_e2) {
                (None,_) => break,
                (Some(e1),None) => {vec.push(e1); break}
                (Some(e1),Some(e2)) => (e1,e2)
            };
            match e1.cmp(&e2) {
                Less => {opt_e1 = s1.next(); opt_e2 = Some(e2); vec.push(e1)}
                Equal => {opt_e1 = s1.next(); opt_e2 = s2.next()}
                Greater => {opt_e1 = Some(e1); opt_e2 = s2.next()}
            }
        }
        for e in s1 {vec.push(e)}
        VecSortedSet(vec)
    }
}

impl<'a,'b,T> Sub<&'b VecSortedSet<T>> for &'a VecSortedSet<T> where T:Ord+Clone {
    type Output = VecSortedSet<T>;
    /// difference with cloned members.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((&s1 - &s2).view_content(), &[1]);
    /// ```
    fn sub(self, other:&VecSortedSet<T>) -> VecSortedSet<T>
    {self.difference(other).cloned().collect()}
}

impl<T> BitXor<VecSortedSet<T>> for VecSortedSet<T> where T:Ord {
    type Output = VecSortedSet<T>;
    /// symmetric difference.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((s1 ^ s2).view_content(), &[1,4]);
    /// ```
    fn bitxor(self, other:VecSortedSet<T>) -> VecSortedSet<T> {
        let mut vec = Vec::new();
        let mut s1 = self.into_iter();
        let mut s2 = other.into_iter();
        let mut opt_e1 = s1.next();
        let mut opt_e2 = s2.next();
        loop {
            let (e1,e2) = match (opt_e1,opt_e2) {
                (None,None) => break,
                (Some(e1),None) => {vec.push(e1); break}
                (None,Some(e2)) => {vec.push(e2); break}
                (Some(e1),Some(e2)) => (e1,e2)
            };
            match e1.cmp(&e2) {
                Less => {opt_e1 = s1.next(); opt_e2 = Some(e2); vec.push(e1)}
                Equal => {opt_e1 = s1.next(); opt_e2 = s2.next()}
                Greater => {opt_e1 = Some(e1); opt_e2 = s2.next(); vec.push(e2)}
            }
        }
        for e in s1 {vec.push(e)}
        for e in s2 {vec.push(e)}
        VecSortedSet(vec)
    }
}

impl<'a,'b,T> BitXor<&'b VecSortedSet<T>> for &'a VecSortedSet<T> where T:Ord+Clone {
    type Output = VecSortedSet<T>;
    /// symmetric difference with cloned members.
    ///
    /// # example
    /// ```
    /// use protocoll::set::VecSortedSet;
    /// let s1:VecSortedSet<_> = vec![1,2,3].into_iter().collect();
    /// let s2:VecSortedSet<_> = vec![2,3,4].into_iter().collect();
    /// assert_eq!((&s1 ^ &s2).view_content(), &[1,4]);
    /// ```
    fn bitxor(self, other:&VecSortedSet<T>) -> VecSortedSet<T>
    {self.symmetric_difference(other).cloned().collect()}
}

/// stolen from BTreeSet
fn cmp_opt<T:Ord>(x:Option<&T>,y:Option<&T>,short:Ordering,long:Ordering) -> Ordering
{match (x, y) {(None,_) => short, (_,None) => long, (Some(x1),Some(y1)) => x1.cmp(y1)}}
#[derive(Clone)]
pub struct Union<'a,T:'a>
{this:Peekable<Iter<'a,T>>,that:Peekable<Iter<'a,T>>}
impl<'a,T> Iterator for Union<'a,T> where T:Ord
{type Item = &'a T;
 fn next(&mut self) -> Option<&'a T>
 {loop {match cmp_opt(self.this.peek(),self.that.peek(),Greater,Less)
        {Less => return self.this.next(),
         Equal => {self.that.next(); return self.this.next()}
         Greater => return self.that.next()}}}
 fn size_hint(&self) -> (usize,Option<usize>)
 {let a = self.this.len(); let b = self.that.len(); (max(a,b),Some(a+b))}}
#[derive(Clone)]
pub struct Intersection<'a,T:'a>
{this:Peekable<Iter<'a,T>>,that:Peekable<Iter<'a,T>>}
impl<'a,T> Iterator for Intersection<'a,T> where T:Ord
{type Item = &'a T;
 fn next(&mut self) -> Option<&'a T>
 {loop {let cmp = match (self.this.peek(),self.that.peek())
        {(None,_) => return None,
         (_,None) => return None,
         (Some(e1),Some(e2)) => e1.cmp(e2)};
        match cmp
        {Less => {self.this.next();}
         Equal => {self.that.next(); return self.this.next()}
         Greater => {self.that.next();}}}}
 fn size_hint(&self) -> (usize,Option<usize>)
 {(0,Some(min(self.this.len(),self.that.len())))}}
#[derive(Clone)]
pub struct Difference<'a,T:'a>
{this:Peekable<Iter<'a,T>>,that:Peekable<Iter<'a,T>>}
impl<'a,T> Iterator for Difference<'a,T> where T:Ord
{type Item = &'a T;
 fn next(&mut self) -> Option<&'a T>
 {loop {match cmp_opt(self.this.peek(),self.that.peek(),Less,Less)
        {Less => return self.this.next(),
         Equal => {self.this.next(); self.that.next();}
         Greater => {self.that.next();}}}}
 fn size_hint(&self) -> (usize,Option<usize>)
 {let a = self.this.len(); let b = self.that.len(); (a.saturating_sub(b),Some(a))}}
#[derive(Clone)]
pub struct SymmetricDifference<'a,T:'a>
{this:Peekable<Iter<'a,T>>,that:Peekable<Iter<'a,T>>}
impl<'a,T> Iterator for SymmetricDifference<'a,T> where T:Ord
{type Item = &'a T;
 fn next(&mut self) -> Option<&'a T>
 {loop {match cmp_opt(self.this.peek(),self.that.peek(),Greater,Less)
        {Less => return self.this.next(),
         Equal => {self.this.next(); self.that.next();}
         Greater => return self.that.next()}}}
 fn size_hint(&self) -> (usize,Option<usize>)
 {(0,Some(self.this.len()+self.that.len()))}}
