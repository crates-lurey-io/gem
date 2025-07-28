use crate::scalar::{
    intensity::Intensity,
    normalize::{NormF32, NormF64, Normalize},
};

/// A trait for types that have a ⚫ ⚪ gray component.
pub trait HasGray<C> {
    /// Creates a new color with the given gray component.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_gray(value: C) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_gray(value);
        color
    }

    /// Creates a new color with the gray component set to the given normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn new_gray_normalized_f32(value: f32) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f32>,
    {
        let mut color = Self::default();
        color.set_gray_normalized_f32(value);
        color
    }

    /// Creates a new color with the gray component set to the given normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn new_gray_normalized_f64(value: f64) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f64>,
    {
        let mut color = Self::default();
        color.set_gray_normalized_f64(value);
        color
    }

    /// Returns the value of the gray component.
    #[must_use]
    fn gray(&self) -> C;

    /// Sets the gray component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_gray(&mut self, value: C);

    /// Returns the gray intensity of the color, as a normalized 32-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn gray_normalized_f32(&self) -> f32
    where
        C: Normalize<NormF32>,
    {
        self.gray().normalize_clamped().into_inner()
    }

    /// Returns the gray intensity of the color, as a normalized 64-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn gray_normalized_f64(&self) -> f64
    where
        C: Normalize<NormF64>,
    {
        self.gray().normalize_clamped().into_inner()
    }

    /// Sets the gray component based on the normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn set_gray_normalized_f32(&mut self, value: f32)
    where
        C: Intensity + Into<f32>,
    {
        self.set_gray(C::from_normalized_f32(value));
    }

    /// Sets the gray component based on the normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn set_gray_normalized_f64(&mut self, value: f64)
    where
        C: Intensity + Into<f64>,
    {
        self.set_gray(C::from_normalized_f64(value));
    }

    /// Converts the color into a new type with the gray component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_gray(self, value: C) -> Self
    where
        Self: Sized,
    {
        let mut color = self;
        color.set_gray(value);
        color
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::gray::GrayAlpha8;

    use super::*;

    #[test]
    fn with_gray_grayalpha8_new_gray() {
        assert_eq!(GrayAlpha8::new_gray(128), GrayAlpha8::new(128, 0));
    }

    #[test]
    fn with_gray_grayalpha8_new_gray_normalized_f32() {
        assert_eq!(
            GrayAlpha8::new_gray_normalized_f32(1.0),
            GrayAlpha8::new(255, 0)
        );
    }

    #[test]
    fn with_gray_grayalpha8_new_gray_normalized_f64() {
        assert_eq!(
            GrayAlpha8::new_gray_normalized_f64(1.0),
            GrayAlpha8::new(255, 0)
        );
    }

    #[test]
    fn with_gray_grayalpha8_get_gray() {
        assert_eq!(GrayAlpha8::new(128, 255).gray(), 128);
    }

    #[test]
    fn with_gray_grayalpha8_set_gray() {
        let mut color = GrayAlpha8::new(128, 255);
        color.set_gray(64);
        assert_eq!(color, GrayAlpha8::new(64, 255));
    }

    #[test]
    fn with_gray_grayalpha8_get_gray_normalized_f32() {
        let color = GrayAlpha8::new(255, 255);
        assert_eq!(color.gray_normalized_f32(), 1.0);
    }

    #[test]
    fn with_gray_grayalpha8_set_gray_normalized_f32() {
        let mut color = GrayAlpha8::new(255, 255);
        color.set_gray_normalized_f32(0.5);
        assert_eq!(color, GrayAlpha8::new(128, 255));
    }

    #[test]
    fn with_gray_grayalpha8_get_gray_normalized_f64() {
        let color = GrayAlpha8::new(255, 255);
        assert_eq!(color.gray_normalized_f64(), 1.0);
    }

    #[test]
    fn with_gray_grayalpha8_set_gray_normalized_f64() {
        let mut color = GrayAlpha8::new(255, 255);
        color.set_gray_normalized_f64(0.5);
        assert_eq!(color, GrayAlpha8::new(128, 255));
    }

    #[test]
    fn with_gray_grayalpha8_with_gray() {
        let color = GrayAlpha8::new(255, 255);
        let new_color = color.with_gray(64);
        assert_eq!(new_color, GrayAlpha8::new(64, 255));
    }
}
