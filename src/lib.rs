#![deny(missing_docs)]
#![deny(warnings)]

#![feature(unboxed_closures)]

//! Exposes `replace_map`, for replacing values at mutable memory locations.

use std::ptr;

/// Replace the value at a mutable memory location with the value
/// produced by the passed in closure.
///
/// Does not create an intermediate value, so is more efficient and
/// ergonomic in cases where producing a value to pass to mem::replace
/// is hard.
///
/// The passed in closure *must* not panic or destructors will run on
/// an instance of T twice.
pub fn replace_map<'a, T, F>(src: &mut T, prod: F)
where F: FnOnce(T) -> T {
    // Read the value, pass it to prod, then write-over src.
    *src = prod(unsafe { ptr::read_and_zero(src as *mut T) });
}

#[test] fn test_works() {
    let mut a = 7u;
    let b = &mut a;

    replace_map(b, |: x: uint| x * 2);
    assert_eq!(*b, 14u);
}

#[test] fn is_panic_safe() {
    static mut DROP_COUNT: uint = 0;
    struct Dropper;

    impl Drop for Dropper {
        fn drop(&mut self) {
            unsafe { DROP_COUNT += 1 }
        }
    }

    std::task::try(proc() {
        let mut a = Dropper;
        let b = &mut a;

        replace_map(b, |: _| panic!("Muahaha"));
    }).unwrap_err();

    assert_eq!(unsafe { DROP_COUNT }, 1);
}

