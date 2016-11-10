//! some basic protocols and implementations for rust collections. inspired by
//! clojure's design, albeit not persistent.

mod map;
pub use map::Map;
mod set;
pub use set::Set;
mod seq;
pub use seq::Seq;
