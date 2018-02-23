# Replace Map

> Replace the value at an `&mut T` using a `FnOnce(T) -> T` closure.

`replace_map` is more ergonomic in cases where producing a value to
pass to mem::replace is hard. Unfortunately, `replace_map` is not safe
in all cases, so care must be taken when using it.

--

Notice: this crate was built pre-rust 1.0, and is deprecated. See [take_mut](https://crates.io/crates/take_mut) for an equivalent crate with a safe API which is supported today on stable rust.
