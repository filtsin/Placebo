//!
use std::ops::{Add, Sub, Mul, Div};

/// Trait for u8, u16, u32, u64 types
pub trait IsU8163264 : Copy + Add + Sub + Mul + Div {}
/// Trait for u8, u16, u32 types
pub trait IsU81632 : IsU8163264 {}

macro_rules! impl_trait {
  ($tr: ident, $ty: ident) => {
    impl $tr for $ty {}
  };
  ($tr: ident, $ty : ident, $($tt: ident), *) => {
    impl_trait!($tr, $ty);
    impl_trait!($tr, $($tt), *);
  }
}

impl_trait!(IsU8163264, u8, u16, u32, u64);
impl_trait!(IsU81632, u8, u16, u32);
