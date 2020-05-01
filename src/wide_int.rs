use crate::raw_int::RawInt;

/// Structure for creating wide integer
///
///
/// ```
/// use placebo::wide_int::WideInt;
/// assert_eq!(std::mem::size_of::<WideInt<u32, 128>>(), 24);
/// assert_eq!(std::mem::size_of::<WideInt<u64, 256>>(), 32);
/// ```
pub struct WideInt<T, const N: usize> {
  buf: RawInt<T, WideIntWrapper<T, N>>
}

struct WideIntWrapper<T, const N: usize> {
  buf: [T; N]
}

impl<T, const N: usize> AsRef<[T; N]> for WideIntWrapper<T, N> {
  fn as_ref(&self) -> &[T; N] { &self.buf }
}
