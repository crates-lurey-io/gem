use crate::{internal::Sealed, scalar::Scalar};

/// Types that have a known intensity range and can be converted from a scalar value.
pub trait Intensity: Sealed {
    /// The minimum value of this intensity type.
    const MIN: Self;

    /// The maximum value of this intensity type.
    const MAX: Self;

    /// Converts a scalar value to this intensity type.
    fn from_scalar(value: &impl Scalar) -> Self;
}

impl Intensity for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;

    fn from_scalar(value: &impl Scalar) -> Self {
        value.to_scaled_u8()
    }
}

impl Intensity for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;

    fn from_scalar(value: &impl Scalar) -> Self {
        value.to_scaled_u16()
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
    fn u8_from_u16() {
        let value: u8 = Intensity::from_scalar(&0.5);
        assert_eq!(value, 128);
    }

    #[test]
    fn u16_min_max() {
        assert_eq!(<u16 as Intensity>::MIN, 0);
        assert_eq!(<u16 as Intensity>::MAX, 65535);
    }

    #[test]
    fn u16_from_u8() {
        let value: u16 = Intensity::from_scalar(&0.5);
        assert_eq!(value, 32768);
    }
}
