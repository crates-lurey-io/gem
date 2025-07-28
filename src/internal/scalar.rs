use core::fmt::Display;

/// A scalar type, optionally with a more restrictive bounds than the numerical data type.
pub trait Scalar<Raw: Scalar = Self>: Sized + Copy + Display + PartialOrd {
    /// The zero value of this type.
    const ZERO: Self;

    /// The minimum value of this type.
    const MIN: Self;

    /// The maximum value of this type.
    const MAX: Self;
}

impl Scalar for u8 {
    const ZERO: Self = 0;
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl Scalar for u16 {
    const ZERO: Self = 0;
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn u8_scalar_zero() {
        assert_eq!(<u8 as Scalar>::ZERO, 0);
    }

    #[test]
    fn u8_scalar_min() {
        assert_eq!(<u8 as Scalar>::MIN, u8::MIN);
    }

    #[test]
    fn u8_scalar_max() {
        assert_eq!(<u8 as Scalar>::MAX, u8::MAX);
    }

    #[test]
    fn u16_scalar_zero() {
        assert_eq!(<u16 as Scalar>::ZERO, 0);
    }

    #[test]
    fn u16_scalar_min() {
        assert_eq!(<u16 as Scalar>::MIN, u16::MIN);
    }

    #[test]
    fn u16_scalar_max() {
        assert_eq!(<u16 as Scalar>::MAX, u16::MAX);
    }

    #[test]
    fn f32_scalar_zero() {
        assert_eq!(<f32 as Scalar>::ZERO, 0.0);
    }

    #[test]
    fn f32_scalar_min() {
        assert_eq!(<f32 as Scalar>::MIN, f32::MIN);
    }

    #[test]
    fn f32_scalar_max() {
        assert_eq!(<f32 as Scalar>::MAX, f32::MAX);
    }
}
