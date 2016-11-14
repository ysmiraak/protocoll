//! some basic protocols and implementations for rust collections. inspired by
//! clojure's design, albeit not persistent.

mod map;
pub use map::Map as Map;
mod set;
pub use set::Set as Set;
mod seq;
pub use seq::Seq as Seq;
mod str;
pub use str::Str as Str;
