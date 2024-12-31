#![allow(clippy::manual_range_contains)]
#![allow(clippy::comparison_chain)]

// TODO use a specialized optimized version of the parser instead of pulling the
// full `awint` dependency, but do fuzz against `awint` in tests

mod signed_macro;
mod signed_macro2;
mod str_conversion;
pub mod traits;

pub use awint;
pub use str_conversion::*;
