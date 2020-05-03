pub mod sign;

use std::marker::PhantomData;
use sign::IsSigned;

/// Structure for raw integer with common operations
///
/// Raw integer does not control size of integer. He just have access to as_ref method
/// of buf object. However RawInt control sign of integer.
/// If S is `Signed` RawInt save sign in 1st bit of slice
///
/// # Template parameters
///
/// T - type with 2 impls: impl AsRef<[U]> + impl AsMut<[U]>
///
/// S - sign type
///
pub(crate) struct RawInt<U, T, S> {
  buf: T,
  marker: PhantomData<U>,
  sign: PhantomData<S>
}

impl<U, T: AsRef<[U]> + AsMut<[U]>, S: IsSigned> RawInt<U, T, S> {
  fn new(buf: T) -> Self {
    RawInt {
      buf,
      marker: PhantomData,
      sign: PhantomData
    }
  }
}

impl<U, T: AsRef<[U]>, S: IsSigned> From<&str> for RawInt<U, T, S> {
  fn from(string: &str) -> Self {
    unimplemented!()
  }
}

impl<U, T: AsRef<[U]>, S: IsSigned> From<u8> for RawInt<U, T, S> {
  fn from(value: u8) -> Self { unimplemented!() }
}
