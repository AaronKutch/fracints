/*
TODO
Does it panic??: This means that code in debug mode will trigger a panic on this case and optimized code will return `fiN::MIN` without a panic.
add `to_string_fraction_radix`
Make errors in proc macro friendly
remove as much experimental stuff as possible
check clippy
check all feature combinations
the following functions:
round
trunc
rec
pow?
sqrt
cos
sin
tan
acos
asin
atan
atan2

fix cos_sin functions to have constant cutoffs

use #[cfg(debug)] for debug builds with the checked operations

- `impl Mul<f32> for fi32` and various others have not been implemented because it could be
  made to return a f32 or fi32. Instead, it is preferred to use functions such as TODO and
  TODO. It is very verbose, but prevents various bugs and ambiguity. Use local private free
  functions or a macro if a particular combination is used often in a module.

We need a full_mul free function into some kind of u256 for fi128 energies
*/

//! **Important notes about this crate:**
//! - It is really easy to overflow with the `/` operator. The `saturating_div` operation
//!   is recommended in many cases.
//! - Do not confuse `fi32()` with `fi32!()`. One of them is a tuple struct and the other is a
//!   macro for entering fractions.
//! - `1 ULP` means the size of 1 increment or decrement of the internal integer, or the
//!   smallest change in value of a fuN or fiN possible
//! - The `{}_mul` `{}_div` functions effectively use truncation when producing the result, and
//!   accuracy can be improved by up to 1 ULP by using `full_mul` followed by rounding instead
//!   of truncation.
//!
//! # Preventing Overflow
//! The functions `wrapping_abs`, `wrapping_neg` (or the unary `-`), `wrapping_mul`, and
//! `wrapping_full_mul` can all be used without possibility of overflow, if the `fiN::MIN` value
//! is guarded against.
//!     - Use `fiN::NEG_ONE` (`fiN!(-1.)`) instead of `fiN::MIN`, except when you want to do
//!       something like `.wrapping_sub(fiN::MIN)` (a trick to add by an exact 1) and have checks
//!       for fiN::MIN
//!     - In the cases of `wrapping_add` and `wrapping_sub`, avoid overflowing unless it will always
//!       wrap around into the valid range (or is checked as a special case).
//!     - Do not use `wrapping_div` directly unless upholding invariants and taking care of fiN::MIN
//!       produced
//!
//! # Examples
//!
//! See the examples in the testcrate in the cargo workspace of the repo containing this crate.

#![cfg_attr(not(feature = "std"), no_std)]

//NOTE: these must stay in a certain order for certain modules to see certain macros
mod normint_parse_error;
#[macro_use]
mod impl_fiN;
#[macro_use]
mod impl_fiN_2;
#[macro_use]
mod fiN;
//#[macro_use]
//mod fuN;
mod constants;

pub use crate::fiN::*;
pub use crate::normint_parse_error::NormintParseError;
