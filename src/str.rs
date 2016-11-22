/// basic protocol for strs;
pub trait Str {
    /// appends char `c`.
    fn inc(self, c:char) -> Self;

    /// pops the last char.
    fn dec(self) -> Self;

    /// appends str `s`.
    fn plus(self, s:&str) -> Self;

    /// `clear`.
    fn zero(self) -> Self;

    /// `shrink_to_fit`.
    fn shrink(self) -> Self;
}

impl Str for String {
    fn inc(mut self, c:char) -> Self
    {self.push(c); self}

    fn dec(mut self) -> Self
    {self.pop(); self}

    fn plus(mut self, s:&str) -> Self
    {self.push_str(s); self}

    fn zero(mut self) -> Self
    {self.clear(); self}

    fn shrink(mut self) -> Self
    {self.shrink_to_fit(); self}
}
