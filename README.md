# Replace Map

> Replace the value at an `&mut T` using a `FnOnce(T) -> T` closure.

`replace_map` is more ergonomic in cases where producing a value to
pass to mem::replace is hard. Unfortunately, `replace_map` is not safe
in all cases, so care must be taken when using it.

