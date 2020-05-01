pub mod sign;

use std::marker::PhantomData;
use sign::IsSigned;

pub(crate) struct RawInt<U, T, S> {
  buf: T,
  marker: PhantomData<U>,
  sign: PhantomData<S>
}

impl<U, T: AsRef<[U]>, S: IsSigned> RawInt<U, T, S> {
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

