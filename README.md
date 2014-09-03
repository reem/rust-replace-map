# Replace Map

> Replace the value at a mutable memory location using a `|T| -> T` closure.

Does not create an intermediate value, so is more efficient and
ergonomic in cases where producing a value to pass to mem::replace
is hard.

