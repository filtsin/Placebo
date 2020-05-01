///
pub struct Signed;
///
pub struct Unsigned;

pub(crate) trait IsSigned {
  fn is_sign() -> bool;
}

impl IsSigned for Signed {
  #[inline]
  fn is_sign() -> bool { true }
}

impl IsSigned for Unsigned {
  #[inline]
  fn is_sign() -> bool { false }
}

