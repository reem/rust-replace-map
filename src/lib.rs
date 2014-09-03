#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Exposes `replace_map`, for replacing values at mutable memory locations.

use std::ptr;

/// Replace the value at a mutable memory location with the value
/// produced by the passed in closure.
///
/// Does not create an intermediate value, so is more efficient and
/// ergonomic in cases where producing a value to pass to mem::replace
/// is hard.
pub fn replace_map<'a, T>(src: &mut T, prod: |T|: 'a -> T) {
    // Read the value, pass it to prod, then write-over src.
    //
    // Safe because the value originally behind src is dropped
    // inside of prod and is then immediately written over.
    unsafe { *src = prod(ptr::read(src as *mut T as *const T)); }
}

