//! Scalar data types and their conversions.
//!
//! ## Examples
//!
//! ```rust
//! use gem::scalar::Scalar;
//!
//! let value_u8: u8 = 128;
//! assert_eq!(value_u8.to_scaled_u16(), 32896);
//! assert_eq!(value_u8.to_normal_f32(), 0.5019607843137255);
//! ```

use crate::internal::{Sealed, math};

mod intensity;
pub use intensity::Intensity;

/// Types that can be converted to normalized and scaled representations.
///
/// Integer types (`u8`, `u16`) are treated as intensities, while floating-point types (`f32`,
/// `f64`) are treated as normalized.
pub trait Scalar: Sealed {
    /// Converts the scalar to a `u8`.
    fn to_scaled_u8(&self) -> u8;

    /// Converts the scalar to a `u16`.
    fn to_scaled_u16(&self) -> u16;

    /// Converts the scalar to a `f32`.
    fn to_normal_f32(&self) -> f32;

    /// Converts the scalar to a `f64`.
    fn to_normal_f64(&self) -> f64;
}

const SCALE_U8: f32 = u8::MAX as f32 / u16::MAX as f32;
const SCALE_U16: f32 = u16::MAX as f32 / u8::MAX as f32;
const FLOAT_32_U8: f32 = u8::MAX as f32;
const FLOAT_32_U16: f32 = u16::MAX as f32;
const FLOAT_64_U8: f64 = u8::MAX as f64;
const FLOAT_64_U16: f64 = u16::MAX as f64;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl Scalar for u8 {
    fn to_scaled_u8(&self) -> u8 {
        *self
    }

    fn to_scaled_u16(&self) -> u16 {
        math::round_f32(f32::from(*self) * SCALE_U16) as u16
    }

    fn to_normal_f32(&self) -> f32 {
        f32::from(*self) / FLOAT_32_U8
    }

    fn to_normal_f64(&self) -> f64 {
        f64::from(*self) / FLOAT_64_U8
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl Scalar for u16 {
    fn to_scaled_u8(&self) -> u8 {
        math::round_f32(f32::from(*self) * SCALE_U8) as u8
    }

    fn to_scaled_u16(&self) -> u16 {
        *self
    }

    fn to_normal_f32(&self) -> f32 {
        f32::from(*self) / FLOAT_32_U16
    }

    fn to_normal_f64(&self) -> f64 {
        f64::from(*self) / FLOAT_64_U16
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl Scalar for f32 {
    fn to_scaled_u8(&self) -> u8 {
        math::round_f32(self * FLOAT_32_U8) as u8
    }

    fn to_scaled_u16(&self) -> u16 {
        math::round_f32(self * FLOAT_32_U16) as u16
    }

    fn to_normal_f32(&self) -> f32 {
        *self
    }

    fn to_normal_f64(&self) -> f64 {
        f64::from(*self)
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl Scalar for f64 {
    fn to_scaled_u8(&self) -> u8 {
        math::round_f64(self * FLOAT_64_U8) as u8
    }

    fn to_scaled_u16(&self) -> u16 {
        math::round_f64(self * FLOAT_64_U16) as u16
    }

    fn to_normal_f32(&self) -> f32 {
        *self as f32
    }

    fn to_normal_f64(&self) -> f64 {
        *self
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn u8_to_scaled_u8() {
        let value: u8 = 128;
        assert_eq!(value.to_scaled_u8(), 128);
    }

    #[test]
    fn u8_to_scaled_u16() {
        let value: u8 = 128;
        assert_eq!(value.to_scaled_u16(), 32896);
    }

    #[test]
    fn u8_to_normal_f32() {
        let value: u8 = 128;
        assert_eq!(value.to_normal_f32(), 0.501_960_8);
    }

    #[test]
    fn u8_to_normal_f64() {
        let value: u8 = 128;
        assert_eq!(value.to_normal_f64(), 0.501_960_784_313_725_5);
    }

    #[test]
    fn u16_to_scaled_u8() {
        let value: u16 = 32896;
        assert_eq!(value.to_scaled_u8(), 128);
    }

    #[test]
    fn u16_to_scaled_u16() {
        let value: u16 = 32896;
        assert_eq!(value.to_scaled_u16(), 32896);
    }

    #[test]
    fn u16_to_normal_f32() {
        let value: u16 = 32896;
        assert_eq!(value.to_normal_f32(), 0.501_960_8);
    }

    #[test]
    fn u16_to_normal_f64() {
        let value: u16 = 32896;
        assert_eq!(value.to_normal_f64(), 0.501_960_784_313_725_5);
    }

    #[test]
    fn f32_to_scaled_u8() {
        let value: f32 = 0.5;
        assert_eq!(value.to_scaled_u8(), 128);
    }

    #[test]
    fn f32_to_scaled_u16() {
        let value: f32 = 0.5;
        assert_eq!(value.to_scaled_u16(), 32768);
    }

    #[test]
    fn f32_to_normal_f32() {
        let value: f32 = 0.5;
        assert_eq!(value.to_normal_f32(), 0.5);
    }

    #[test]
    fn f32_to_normal_f64() {
        let value: f32 = 0.5;
        assert_eq!(value.to_normal_f64(), 0.5);
    }

    #[test]
    fn f64_to_scaled_u8() {
        let value: f64 = 0.5;
        assert_eq!(value.to_scaled_u8(), 128);
    }

    #[test]
    fn f64_to_scaled_u16() {
        let value: f64 = 0.5;
        assert_eq!(value.to_scaled_u16(), 32768);
    }

    #[test]
    fn f64_to_normal_f32() {
        let value: f64 = 0.5;
        assert_eq!(value.to_normal_f32(), 0.5);
    }

    #[test]
    fn f64_to_normal_f64() {
        let value: f64 = 0.5;
        assert_eq!(value.to_normal_f64(), 0.5);
    }
}
