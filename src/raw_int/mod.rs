//!
pub mod num_traits;
pub mod sign;

use num_traits::IsU81632;
use sign::IsSigned;
use std::marker::PhantomData;

use make_const::const_if_feature_enabled;

/// Structure for raw integer with common operations
///
/// Raw integer does not control size of integer. He just have access to as_ref+as_mut methods
/// of buf object. However RawInt control sign of integer.
/// If S is `Signed` RawInt save sign in 1st bit of slice (1 - positive, 0 - negative)
///
/// # Template parameters
///
/// T - type with 2 impls: impl AsRef<[U]> + impl AsMut<[U]>
///
/// S - sign type (impl sign::IsSigned)
///
pub(crate) struct RawInt<U, T, S> {
    buf: T,
    marker: PhantomData<U>,
    sign: PhantomData<S>,
}


impl<U, T: AsRef<[U]> + AsMut<[U]>, S: IsSigned> RawInt<U, T, S> {
    #[const_if_feature_enabled]
    fn new(buf: T) -> Self {
        RawInt {
            buf,
            marker: PhantomData,
            sign: PhantomData,
        }
    }
}
