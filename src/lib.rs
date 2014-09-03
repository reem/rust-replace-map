#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

#![feature(unboxed_closures, overloaded_calls)]

//! Exposes `replace_map`, for replacing values at mutable memory locations.

use std::ptr;

/// Replace the value at a mutable memory location with the value
/// produced by the passed in closure.
///
/// Does not create an intermediate value, so is more efficient and
/// ergonomic in cases where producing a value to pass to mem::replace
/// is hard.
pub fn replace_map<'a, T, F>(src: &mut T, prod: F)
where F: |: T| -> T {
    // Read the value, pass it to prod, then write-over src.
    //
    // Safe because the value originally behind src is dropped
    // inside of prod and is then immediately written over.
    unsafe { *src = prod(ptr::read(src as *mut T as *const T)); }
}

#[test] fn test_works() {
    let mut a = 7u;
    let b = &mut a;

    replace_map(b, |: x: uint| x * 2);
    assert_eq!(*b, 128u);
}

