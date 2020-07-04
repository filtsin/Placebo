//!
#![warn(missing_docs)]
#![cfg_attr(not(feature = "generic-array"), feature(const_generics))]

mod raw_int;

#[cfg(not(feature = "generic-array"))]
pub mod wide_int;

#[cfg(feature = "generic-array")]
pub mod wide_int_gen_array;
