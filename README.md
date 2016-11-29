# protocoll

some basic protocols and their implementations for `rust` collections.

inspired by `clojure`'s design, albeit not persistent.

[cargo crate](https://crates.io/crates/protocoll)

[api doc](https://ysmiraak.github.io/protocoll/)

these protocols are intended for containing the unavoidable complexities in the
`rust` language through the simplicity of functional programming as is strongly
manifested in `clojure`, which does everything with only a handful of functions
operating on several kinds of collections.

they try to fulfill that intention by supporting just a few functions which
either take type `T` for creation and type `&T` for utilization, and to reject
the abominations which take type `&mut T` for destructive alteration.

however, for efficiency reasons, i find that i have to support the
abominations. for example, i cannot consume a value stored in a map to produce a
new value of the same type to replace the old one without first removing the
entry and then re-inserting the entry. on the other hand, the abominations can
just mutate the value in place. so if you know how to temporarily move out a
value without `rust` complaining, please let me know :)

the library is currently experimental and extremely unstable.
