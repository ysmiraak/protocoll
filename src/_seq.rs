use std::collections::{VecDeque,BinaryHeap};

/// basic protocol for seqs.
pub trait Seq<T> where Self:Sized {
    /// a seq maps from indices to items. O(n) for `BinaryHeap`.
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>;

    /// like `clojure`'s [`conj`](http://clojuredocs.org/clojure.core/conj)
    ///
    /// adds item `i`. both `Vec` and `VecDeque` grows to the right.
    fn inc(self, i:T) -> Self;

    /// like `clojure`'s [`pop`](http://clojuredocs.org/clojure.core/pop) for
    /// vectors and queues.
    ///
    /// removes an item. for `Vec` it's the last one; for `VecDeque` the first;
    /// for `BinaryHeap` it's the greatest one.
    fn dec(self) -> Self;

    /// like `clojure`'s [`into`](http://clojuredocs.org/clojure.core/into)
    ///
    /// pours another collection into this one.
    fn plus<I>(self, coll:I) -> Self where I:IntoIterator<Item = T>
    {coll.into_iter().fold(self, Seq::inc)}

    /// `clear`.
    fn zero(self) -> Self;

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;
}

impl<T> Seq<T> for Vec<T> {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    {Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    {self.push(i); self}

    fn dec(mut self) -> Self
    {self.pop(); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}
}

impl<T> Seq<T> for VecDeque<T> {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    {Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    {self.push_back(i); self}

    fn dec(mut self) -> Self
    {self.pop_front(); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}
}

impl<T> Seq<T> for BinaryHeap<T> where T:Ord {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    {Box::new(move |i| self.iter().skip(i).next())}

    fn inc(mut self, i:T) -> Self
    {self.push(i); self}

    fn dec(mut self) -> Self
    {self.pop(); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}
}
