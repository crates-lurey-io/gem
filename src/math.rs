//! Helper crate for crate-dependent math operations.

#[cfg(all(not(feature = "std"), feature = "libm"))]
mod impl_libm;

#[cfg(all(not(feature = "std"), feature = "libm"))]
use impl_libm as impls;

#[cfg(feature = "std")]
mod impl_std;

#[cfg(feature = "std")]
use impl_std as impls;

/// Rounds a 32-bit floating point value.
#[inline]
pub fn round_f32(value: f32) -> f32 {
    impls::round_f32(value)
}

/// Rounds a 64-bit floating point value.
#[inline]
pub fn round_f64(value: f64) -> f64 {
    impls::round_f64(value)
}
