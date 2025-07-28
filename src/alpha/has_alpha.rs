use crate::scalar::{
    intensity::Intensity,
    normalize::{NormF32, NormF64, Normalize},
};

/// A trait for types that have an alpha (transparency) component.
pub trait HasAlpha<C> {
    /// Creates a new color with the given alpha component.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_alpha(value: C) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_alpha(value);
        color
    }

    /// Creates a new color with the alpha component set to the given normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn new_alpha_normalized_f32(value: f32) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f32>,
    {
        Self::new_alpha(C::from_normalized_f32(value))
    }

    /// Creates a new color with the alpha component set to the given normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn new_alpha_normalized_f64(value: f64) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f64>,
    {
        Self::new_alpha(C::from_normalized_f64(value))
    }

    /// Returns the value of the alpha component.
    #[must_use]
    fn alpha(&self) -> C;

    /// Sets the alpha component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_alpha(&mut self, value: C);

    /// Returns the alpha intensity of the color, as a normalized 32-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn alpha_normalized_f32(&self) -> f32
    where
        C: Normalize<NormF32>,
    {
        self.alpha().normalize_clamped().into_inner()
    }

    /// Returns the alpha intensity of the color, as a normalized 64-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn alpha_normalized_f64(&self) -> f64
    where
        C: Normalize<NormF64>,
    {
        self.alpha().normalize_clamped().into_inner()
    }

    /// Sets the alpha component based on the normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn set_alpha_normalized_f32(&mut self, value: f32)
    where
        C: Intensity + Into<f32>,
    {
        self.set_alpha(C::from_normalized_f32(value));
    }

    /// Sets the alpha component based on the normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn set_alpha_normalized_f64(&mut self, value: f64)
    where
        C: Intensity + Into<f64>,
    {
        self.set_alpha(C::from_normalized_f64(value));
    }

    /// Converts the color into a new type with the alpha component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_alpha(self, value: C) -> Self
    where
        Self: Sized,
    {
        let mut color = self;
        color.set_alpha(value);
        color
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::gray::GrayAlpha8;

    use super::*;

    #[test]
    fn with_alpha_grayalpha8_new_alpha() {
        assert_eq!(GrayAlpha8::new_alpha(128), GrayAlpha8::new(0, 128));
    }

    #[test]
    fn with_alpha_grayalpha8_new_alpha_normalized_f32() {
        assert_eq!(
            GrayAlpha8::new_alpha_normalized_f32(1.0),
            GrayAlpha8::new(0, 255)
        );
    }

    #[test]
    fn with_alpha_grayalpha8_new_alpha_normalized_f64() {
        assert_eq!(
            GrayAlpha8::new_alpha_normalized_f64(1.0),
            GrayAlpha8::new(0, 255)
        );
    }

    #[test]
    fn with_alpha_grayalpha8_get_alpha() {
        assert_eq!(GrayAlpha8::new(128, 255).alpha(), 255);
    }

    #[test]
    fn with_alpha_grayalpha8_set_alpha() {
        let mut color = GrayAlpha8::new(128, 255);
        color.set_alpha(64);
        assert_eq!(color, GrayAlpha8::new(128, 64));
    }

    #[test]
    fn with_alpha_grayalpha8_get_alpha_normalized_f32() {
        let color = GrayAlpha8::new(128, 255);
        assert_eq!(color.alpha_normalized_f32(), 1.0);
    }

    #[test]
    fn with_alpha_grayalpha8_set_alpha_normalized_f32() {
        let mut color = GrayAlpha8::new(128, 255);
        color.set_alpha_normalized_f32(0.5);
        assert_eq!(color, GrayAlpha8::new(128, 128));
    }

    #[test]
    fn with_alpha_grayalpha8_get_alpha_normalized_f64() {
        let color = GrayAlpha8::new(128, 255);
        assert_eq!(color.alpha_normalized_f64(), 1.0);
    }

    #[test]
    fn with_alpha_grayalpha8_set_alpha_normalized_f64() {
        let mut color = GrayAlpha8::new(128, 255);
        color.set_alpha_normalized_f64(0.5);
        assert_eq!(color, GrayAlpha8::new(128, 128));
    }

    #[test]
    fn with_alpha_grayalpha8_with_alpha() {
        let color = GrayAlpha8::new(128, 255);
        let new_color = color.with_alpha(64);
        assert_eq!(new_color, GrayAlpha8::new(128, 64));
    }
}
