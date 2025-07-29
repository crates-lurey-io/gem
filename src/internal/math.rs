//! Helper crate for crate-dependent math operations.

#[cfg(all(not(feature = "std"), feature = "libm"))]
#[path = "math/impl_libm.rs"]
mod impls;

#[cfg(feature = "std")]
#[path = "math/impl_std.rs"]
mod impls;

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
#[path = "math/impl_fallback.rs"]
mod impls;

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

#[cfg(test)]
mod impl_libm;

#[cfg(test)]
#[allow(clippy::duplicate_mod)]
mod impl_std;

#[cfg(test)]
mod impl_fallback;

#[cfg(test)]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::float_cmp
)]
mod tests {
    use super::*;

    #[test]
    fn libm_round_f32() {
        assert_eq!(impl_libm::round_f32(0.0) as u8, 0);
        assert_eq!(impl_libm::round_f32(1.25) as u8, 1);
        assert_eq!(impl_libm::round_f32(1.50) as u8, 2);
        assert_eq!(impl_libm::round_f32(1.75) as u8, 2);

        assert_eq!(impl_libm::round_f32(-0.0) as i8, 0);
        assert_eq!(impl_libm::round_f32(-1.25) as i8, -1);
        assert_eq!(impl_libm::round_f32(-1.50) as i8, -2);
        assert_eq!(impl_libm::round_f32(-1.75) as i8, -2);
    }

    #[test]
    fn libm_round_f64() {
        assert_eq!(impl_libm::round_f64(0.0) as u8, 0);
        assert_eq!(impl_libm::round_f64(1.25) as u8, 1);
        assert_eq!(impl_libm::round_f64(1.50) as u8, 2);
        assert_eq!(impl_libm::round_f64(1.75) as u8, 2);

        assert_eq!(impl_libm::round_f64(-0.0) as i8, 0);
        assert_eq!(impl_libm::round_f64(-1.25) as i8, -1);
        assert_eq!(impl_libm::round_f64(-1.50) as i8, -2);
        assert_eq!(impl_libm::round_f64(-1.75) as i8, -2);
    }

    #[test]
    fn std_round_f32() {
        assert_eq!(impl_std::round_f32(0.0) as u8, 0);
        assert_eq!(impl_std::round_f32(1.25) as u8, 1);
        assert_eq!(impl_std::round_f32(1.50) as u8, 2);
        assert_eq!(impl_std::round_f32(1.75) as u8, 2);

        assert_eq!(impl_std::round_f32(-0.0) as i8, 0);
        assert_eq!(impl_std::round_f32(-1.25) as i8, -1);
        assert_eq!(impl_std::round_f32(-1.50) as i8, -2);
        assert_eq!(impl_std::round_f32(-1.75) as i8, -2);
    }

    #[test]
    fn std_round_f64() {
        assert_eq!(impl_std::round_f64(0.0) as u8, 0);
        assert_eq!(impl_std::round_f64(1.25) as u8, 1);
        assert_eq!(impl_std::round_f64(1.50) as u8, 2);
        assert_eq!(impl_std::round_f64(1.75) as u8, 2);

        assert_eq!(impl_std::round_f64(-0.0) as i8, 0);
        assert_eq!(impl_std::round_f64(-1.25) as i8, -1);
        assert_eq!(impl_std::round_f64(-1.50) as i8, -2);
        assert_eq!(impl_std::round_f64(-1.75) as i8, -2);
    }

    #[test]
    fn fallback_round_f32() {
        assert_eq!(impl_fallback::round_f32(0.0) as u8, 0);
        assert_eq!(impl_fallback::round_f32(1.25) as u8, 1);
        assert_eq!(impl_fallback::round_f32(1.50) as u8, 2);
        assert_eq!(impl_fallback::round_f32(1.75) as u8, 2);

        assert_eq!(impl_fallback::round_f32(-0.0) as i8, 0);
        assert_eq!(impl_fallback::round_f32(-1.25) as i8, -1);
        assert_eq!(impl_fallback::round_f32(-1.50) as i8, -2);
        assert_eq!(impl_fallback::round_f32(-1.75) as i8, -2);
    }

    #[test]
    fn fallback_round_f64() {
        assert_eq!(impl_fallback::round_f64(0.0) as u8, 0);
        assert_eq!(impl_fallback::round_f64(1.25) as u8, 1);
        assert_eq!(impl_fallback::round_f64(1.50) as u8, 2);
        assert_eq!(impl_fallback::round_f64(1.75) as u8, 2);

        assert_eq!(impl_fallback::round_f64(-0.0) as i8, 0);
        assert_eq!(impl_fallback::round_f64(-1.25) as i8, -1);
        assert_eq!(impl_fallback::round_f64(-1.50) as i8, -2);
        assert_eq!(impl_fallback::round_f64(-1.75) as i8, -2);
    }
}
