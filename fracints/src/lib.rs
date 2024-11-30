//! Fracints
//!
//! See the README.md for more.

#![cfg_attr(not(feature = "std"), no_std)]

mod constants;
mod impl_signed;
mod traits;

pub use fracints_internals::FracintSerdeError;
pub use fracints_macros::*;
pub use traits::Fracint;

pub use crate::impl_signed::*;
