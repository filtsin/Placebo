use std::marker::PhantomData;

pub(crate) struct RawInt<U, T> {
  buf: T,
  marker: PhantomData<U>,
}

impl<U, T: AsRef<[U]>> RawInt<U, T> {
  fn new(buf: T) -> Self {
    RawInt {
      buf,
      marker: PhantomData,
    }
  }
}

impl<U, T: AsRef<[U]>> From<&str> for RawInt<U, T> {
  fn from(string: &str) -> Self {
    unimplemented!()
  }
}

