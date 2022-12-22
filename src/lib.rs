//! This crate provides [likely] and [unlikely] functions that work as compiler hints for branching.
//! The functions themselves should mostly be optimized out by the compiler. They are taken directly from the
//! [hashbrown](https://crates.io/crates/hashbrown) crate, all credit belongs to them.

// Branch prediction hint. This is currently only available on nightly.
#[cfg(feature = "nightly")]
use core::intrinsics::{likely, unlikely};

// On stable we can use #[cold] to get a equivalent effect: this attributes
// suggests that the function is unlikely to be called
#[cfg(not(feature = "nightly"))]
#[inline]
#[cold]
fn cold() {}

/// This function works as a compiler hint that the boolean it's
/// given is likely to be true.
/// # Example
/// ```
/// # use branch_hints::likely;
/// use rand::prelude::*;
/// if likely(rand::random::<u64>() > 0) {
///     println!("It's nonzero!");
/// }
/// ```
#[cfg(not(feature = "nightly"))]
#[inline]
pub fn likely(b: bool) -> bool {
    if !b {
        cold();
    }
    b
}

/// This function works as a compiler hint that the boolean it's
/// given is likely to be false.
/// # Example
/// ```
/// # use branch_hints::unlikely;
/// use rand::prelude::*;
/// if unlikely(rand::random::<u64>() == 42) {
///     println!("We didn't find the meaning of life, the universe, and everything");
/// }
/// ```
#[cfg(not(feature = "nightly"))]
#[inline]
pub fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}