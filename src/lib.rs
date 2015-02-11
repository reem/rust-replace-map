#![feature(core)]
#![cfg_attr(test, feature(std_misc))]

#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]


//! Exposes `replace_map`, for replacing values at mutable memory locations.

use std::ptr;

/// Replace the value at a mutable memory location with the value
/// produced by the passed in closure.
///
/// Does not create an intermediate value, so is more efficient and
/// ergonomic in cases where producing a value to pass to mem::replace
/// is hard.
///
/// This is not a totally safe function, it has some curious edge cases.
/// Generally you should only pass in a reference the data behind
/// that reference owns itself. An example may be helpful:
///
/// Unsafe Usage:
///
/// ```ignore
/// struct Foo { num: Box<u32> }
/// struct Bar { foo: Foo }
///
/// impl Drop for Bar {
///     fn drop(&mut self) {
///         *self.foo.num
///     }
/// }
///
/// let mut b = Bar { foo: Foo { num: Box::new(123) } };
///
/// // replace_map will zero b.foo.num when it reads it and passes
/// // it to the closure. The closure then panics, leaving b.foo.num
/// // set to 0, which then causes a null-pointer dereference in b's
/// // destructors.
/// unsafe { replace_map(&mut b.foo.num, |_| panic!()); }
/// ```
pub unsafe fn replace_map<'a, T, F>(src: &mut T, prod: F)
where F: FnOnce(T) -> T {
    // Read the value, pass it to prod, then write-over src.
    *src = prod(ptr::read_and_zero(src as *mut T));
}

#[test] fn test_works() {
    let mut a = 7;
    let b = &mut a;

    unsafe { replace_map(b, |x: usize| x * 2) };
    assert_eq!(*b, 14);
}

#[test] fn is_partially_panic_safe() {
    static mut DROP_COUNT: usize = 0;
    struct Dropper;

    impl Drop for Dropper {
        fn drop(&mut self) {
            unsafe { DROP_COUNT += 1 }
        }
    }

    std::thread::Thread::scoped(move || {
        let mut a = Dropper;
        let b = &mut a;

        unsafe { replace_map(b, |_| panic!("Muahaha")); }
    }).join().unwrap_err();

    assert_eq!(unsafe { DROP_COUNT }, 1);
}

