//! Fracints
//!
//! Note there are "rand_support" and "serde_support" features.
//!
//! See the library documentation of [awint_core] for more information of how to
//! deal with fixed point numbers.
//!
//! See the README.md for more.

#![cfg_attr(not(feature = "std"), no_std)]

mod constants;
mod impl_signed;
#[doc(hidden)]
pub mod internal;

pub use fracints_internals::{FracintSerdeError, traits::*};
pub use fracints_macros::*;

pub use crate::impl_signed::*;

pub mod prelude {
    pub use fracints_internals::traits::*;
    pub use fracints_macros::*;

    pub use crate::impl_signed::*;
}
