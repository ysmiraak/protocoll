//! some basic protocols and implementations for rust collections. inspired by
//! clojure's design, albeit not persistent.

mod _map;
mod vec_sorted_map;

pub use _map::{Map,MapMut};
pub mod map {
    pub use vec_sorted_map::VecSortedMap;
}

mod _set;
mod vec_sorted_set;
pub use _set::Set;
pub mod set {
    pub use vec_sorted_set::VecSortedSet;
}

mod _seq;
pub use _seq::Seq;

mod _str;
pub use _str::Str;
