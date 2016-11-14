/// basic protocol for strs;
pub trait Str {
    /// pushs `c`.
    fn inc(self, c:char) -> Self;

    /// pops the last char.
    fn dec(self) -> Self;

    /// appends `s`.
    fn absorb(self, s:&str) -> Self;

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;

    /// `clear`.
    fn empty(self) -> Self;
}

impl Str for String {
    fn inc(mut self, c:char) -> Self
    { self.push(c); self }

    fn dec(mut self) -> Self
    { self.pop(); self }

    fn absorb(mut self, s:&str) -> Self
    { self.push_str(s); self}

    fn shrink(mut self) -> Self
    { self.shrink_to_fit(); self }

    fn empty(self) -> Self
    { String::from("") }
}
