pub trait IsBuiltNum {}
pub trait IsBuiltUnsigned {}

macro_rules! impl_trait {
  ($tr: ident, $ty: ident) => {
    impl $tr for $ty {}
  };
  ($tr: ident, $ty : ident, $($tt: ident), *) => {
    impl_trait!($tr, $ty);
    impl_trait!($tr, $($tt), *);
  }
}

impl_trait!(IsBuiltNum, i8, u8, i16, u16, i32, u32, i64, u64);
impl_trait!(IsBuiltUnsigned, u8, u16, u32, u64);
