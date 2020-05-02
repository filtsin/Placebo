use crate::raw_int::RawInt;
pub use crate::raw_int::sign::{Signed, Unsigned};

/// Structure for creating wide integer
///
/// # Template parameters
///
/// T should be a based type for representing number. It is should be unsigned for correct overflow
/// (u8, u16, u32, u64).
///
/// S for representing signed or unsigned number (Use Signed or Unsigned types).
///
/// N is count of bits
///
/// ```
/// use placebo::wide_int::{WideInt, Signed, Unsigned};
/// 
/// assert_eq!(std::mem::size_of::<WideInt<u32, Signed, 128>>(), 24);
/// assert_eq!(std::mem::size_of::<WideInt<u64, Unsigned, 256>>(), 32);
/// ```
pub struct WideInt<T, S, const N: usize> {
  buf: RawInt<T, WideIntWrapper<T, N>, S>
}

struct WideIntWrapper<T, const N: usize> {
  /// Not current implementation...
  buf: [T; N]
}

impl<T, const N: usize> AsRef<[T]> for WideIntWrapper<T, N> {
  fn as_ref(&self) -> &[T] { &self.buf }
}

impl<T, const N: usize> AsMut<[T]> for WideIntWrapper<T, N> {
  fn as_mut(&mut self) -> &mut [T] { &mut self.buf }
}
