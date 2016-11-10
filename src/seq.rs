use std::collections::{VecDeque,BinaryHeap};

/// basic protocol for seqs.
pub trait Seq<T> where Self:Sized {
    /// a seq maps from indices to items. O(n) for `BinaryHeap`. undefined for `String`.
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>;

    /// adds item `i`. both `Vec` and `VecDeque` grows to the right.
    fn inc(self, i:T) -> Self;

    /// removes an item. for `Vec` it's the last one; for `VecDeque` the first;
    /// for `BinaryHeap` it's the greatest one. see also `Coll::inc`.
    fn dec(self) -> Self;

    /// take another collection into this one.
    fn absorb<I>(self, coll:I) -> Self where Self:Sized, I:IntoIterator<Item = T>
    { coll.into_iter().fold(self, Seq::inc)}

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;

    /// `clear`.
    fn empty(self) -> Self;
}

impl Seq<char> for String {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a char> + 'a>
    { Box::new(|_| Option::None)}

    fn inc(mut self, i:char) -> Self
    { self.push(i); self }

    fn dec(mut self) -> Self
    { self.pop(); self }

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(self) -> Self
    { String::from("") }
}

impl<T> Seq<T> for Vec<T> {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    { Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    { self.push(i); self }

    fn dec(mut self) -> Self
    { self.pop(); self }

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(mut self) -> Self
    { self.clear(); self }
}

impl<T> Seq<T> for VecDeque<T> {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    { Box::new(move |i| self.get(i))}

    fn inc(mut self, i:T) -> Self
    { self.push_back(i); self }

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(mut self) -> Self
    { self.clear(); self }

    fn dec(mut self) -> Self
    { self.pop_front(); self }
}

impl<T> Seq<T> for BinaryHeap<T> where T:Ord {
    fn fun<'a>(&'a self) -> Box<Fn(usize) -> Option<&'a T> + 'a>
    { Box::new(move |i| self.iter().skip(i).next())}

    fn inc(mut self, i:T) -> Self
    { self.push(i); self }

    fn dec(mut self) -> Self
    { self.pop(); self }

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(mut self) -> Self
    { self.clear(); self }
}


// fn main() {
//     let v:Vec<_> = (0..9).collect();
//     let d:VecDeque<_> = (0..9).collect();
//     let b:BinaryHeap<_> = (0..9).collect();

//     println!("{:?}",v.fun()(4));
//     println!("{:?}",d.fun()(4));
//     println!("{:?}",b.fun()(4));

//     println!("{:?}",v.dec());
//     println!("{:?}",d.dec());
//     println!("{:?}",b.dec());

//     let s1 = String::from("abc").inc('d');
//     let s2 = "efg";

//     println!("{}", s1.absorb(s2.chars()));
// }
