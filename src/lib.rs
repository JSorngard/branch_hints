//! This crate provides [`likely`] and [`unlikely`] functions that work as compiler hints for branching in stable rust.
//! They are taken directly from the [`hashbrown`](https://crates.io/crates/hashbrown) crate, all credit belongs to them.
//! These functions enable constructions like
//! ```
//! # use branch_hints::likely;
//! # let condition = true;
//! if likely(condition) {
//!     // main code
//! } else {
//!     // error handling code
//! }
//! ```
//! and they should be optimized away by the compiler.
//!
//! # Notes
//! This is a very minimal crate. If you want more comprehensive functionality, take a look at the [`likely_stable`](https://docs.rs/likely_stable/latest/likely_stable/) crate.

#![no_std]

// The branch hint functions `core::intrinsics::likely` and `core::intrinsics::unlikely` are only available on nightly.
// On stable we can use #[cold] to get an equivalent effect: this attribute
// suggests that the function is unlikely to be called. This function should
// be optimized away by the compiler.
#[inline]
#[cold]
fn cold() {}

/// Returns the boolean passed to it while hinting to the compiler that it is likely to be true.
/// This function brings [`likely`](core::intrinsics::likely) to stable Rust.
/// # Example
/// ```
/// # use branch_hints::likely;
/// use rand::prelude::*;
/// if likely(rand::random::<u64>() > 0) {
///     println!("It's nonzero!");
/// }
/// ```
#[inline]
pub fn likely(b: bool) -> bool {
    if !b {
        cold();
    }
    b
}

/// Returns the boolean passed to it while hinting to the compiler that it is unlikely to be true.
/// This function brings [`unlikely`](core::intrinsics::unlikely) to stable Rust.
/// # Example
/// ```
/// # use branch_hints::unlikely;
/// use rand::prelude::*;
/// if unlikely(rand::random::<u64>() == 42) {
///     println!("We found the meaning of life, the universe, and everything");
/// }
/// ```
#[inline]
pub fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_correctness_of_likely() {
        assert_eq!(likely(true), true);
        assert_eq!(likely(false), false);
    }

    #[test]
    fn check_correctness_of_unlikely() {
        assert_eq!(unlikely(true), true);
        assert_eq!(unlikely(false), false);
    }
}
