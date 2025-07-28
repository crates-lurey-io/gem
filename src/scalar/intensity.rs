use crate::math;

/// A trait that represents scalar data types used as an intensity value.
pub trait Intensity {
    /// The minimum value of this intensity type.
    const MIN: Self;

    /// The maximum value of this intensity type.
    const MAX: Self;

    /// Converts a 32-bit normalized floating point value to this intensity type.
    ///
    /// If the value is outside the range [0.0, 1.0], it will be clamped to this range.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn from_normalized_f32(value: f32) -> Self;

    /// Converts a 64-bit normalized floating point value to this intensity type.
    ///
    /// If the value is outside the range [0.0, 1.0], it will be clamped to this range.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn from_normalized_f64(value: f64) -> Self;
}

impl Intensity for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;

    #[cfg(any(feature = "std", feature = "libm"))]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from_normalized_f32(value: f32) -> Self {
        math::round_f32(value * f32::from(Self::MAX)) as Self
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from_normalized_f64(value: f64) -> Self {
        math::round_f64(value * f64::from(Self::MAX)) as Self
    }
}

impl Intensity for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;

    #[cfg(any(feature = "std", feature = "libm"))]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from_normalized_f32(value: f32) -> Self {
        math::round_f32(value * f32::from(Self::MAX)) as Self
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from_normalized_f64(value: f64) -> Self {
        math::round_f64(value * f64::from(Self::MAX)) as Self
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn u8_min_max() {
        assert_eq!(<u8 as Intensity>::MIN, 0);
        assert_eq!(<u8 as Intensity>::MAX, 255);
    }

    #[test]
    fn u16_min_max() {
        assert_eq!(<u16 as Intensity>::MIN, 0);
        assert_eq!(<u16 as Intensity>::MAX, 65535);
    }
}
