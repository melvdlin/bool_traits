#![no_std]
#![allow(private_interfaces)]

//! Use boolean expressions as trait bounds.
//! # Examples
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! use bool_traits::*;
//!
//! fn create_small_u8_array<const N: usize>() -> [u8; N]
//! where
//!     (): True<{ N <= 4 }>,
//! {
//!     [0; N]
//! }
//!
//! // this compiles:
//! let array = create_small_u8_array::<3>();
//! // this does not:
//! let array = create_small_u8_array::<5>();
//! ```

struct Seal;

/// Require `B` to evaluate to `true`.
pub trait True<const B: bool> {
    #[doc(hidden)]
    fn sealed() -> Seal;
}

/// Require `B` to evaluate to `false`.
pub trait False<const B: bool> {
    #[doc(hidden)]
    fn sealed() -> Seal;
}

impl<T> True<true> for T {
    fn sealed() -> Seal {
        Seal
    }
}

impl<T> False<false> for T {
    fn sealed() -> Seal {
        Seal
    }
}
