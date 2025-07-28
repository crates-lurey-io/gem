use crate::scalar::{
    intensity::Intensity,
    normalize::{NormF32, NormF64, Normalize},
};

/// A trait for types that have a 🔵 blue component.
pub trait WithBlue<C> {
    /// Creates a new color with the given blue component.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_blue(value: C) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_blue(value);
        color
    }

    /// Creates a new color with the blue component set to the given normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_blue_normalized_f32(value: f32) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f32>,
    {
        let mut color = Self::default();
        color.set_blue_normalized_f32(value);
        color
    }

    /// Creates a new color with the blue component set to the given normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_blue_normalized_f64(value: f64) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f64>,
    {
        let mut color = Self::default();
        color.set_blue_normalized_f64(value);
        color
    }

    /// Returns the value of the blue component.
    #[must_use]
    fn blue(&self) -> C;

    /// Sets the blue component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_blue(&mut self, value: C);

    /// Returns the blue intensity of the color, as a normalized 32-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn blue_normalized_f32(&self) -> f32
    where
        C: Normalize<NormF32>,
    {
        self.blue().normalize_clamped().into_inner()
    }

    /// Returns the blue intensity of the color, as a normalized 64-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn blue_normalized_f64(&self) -> f64
    where
        C: Normalize<NormF64>,
    {
        self.blue().normalize_clamped().into_inner()
    }

    /// Sets the blue component based on the normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    fn set_blue_normalized_f32(&mut self, value: f32)
    where
        C: Intensity + Into<f32>,
    {
        self.set_blue(C::from_normalized_f32(value));
    }

    /// Sets the blue component based on the normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    fn set_blue_normalized_f64(&mut self, value: f64)
    where
        C: Intensity + Into<f64>,
    {
        self.set_blue(C::from_normalized_f64(value));
    }

    /// Converts the color into a new type with the blue component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_blue(self, value: C) -> Self
    where
        Self: Copy + Clone,
    {
        let mut color = self;
        color.set_blue(value);
        color
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::rgb::Rgb888;

    use super::*;

    #[test]
    fn with_blue_rgb888_new_blue() {
        assert_eq!(Rgb888::new_blue(255), Rgb888::new(0, 0, 255));
    }

    #[test]
    fn with_blue_rgb888_new_blue_normalized_f32() {
        assert_eq!(Rgb888::new_blue_normalized_f32(1.0), Rgb888::new(0, 0, 255));
    }

    #[test]
    fn with_blue_rgb888_new_blue_normalized_f64() {
        assert_eq!(Rgb888::new_blue_normalized_f64(1.0), Rgb888::new(0, 0, 255));
    }

    #[test]
    fn with_blue_rgb888_get_blue() {
        assert_eq!(Rgb888::new(0, 0, 255).blue(), 255);
    }

    #[test]
    fn with_blue_rgb888_set_blue() {
        let mut color = Rgb888::new(0, 0, 255);
        color.set_blue(128);
        assert_eq!(color, Rgb888::new(0, 0, 128));
    }

    #[test]
    fn with_blue_rgb888_get_blue_normalized_f32() {
        let color = Rgb888::new(0, 0, 255);
        assert_eq!(color.blue_normalized_f32(), 1.0);
    }

    #[test]
    fn with_blue_rgb888_set_blue_normalized_f32() {
        let mut color = Rgb888::new(0, 0, 255);
        color.set_blue_normalized_f32(0.5);
        assert_eq!(color, Rgb888::new(0, 0, 128));
    }

    #[test]
    fn with_blue_rgb888_get_blue_normalized_f64() {
        let color = Rgb888::new(0, 0, 255);
        assert_eq!(color.blue_normalized_f64(), 1.0);
    }

    #[test]
    fn with_blue_rgb888_set_blue_normalized_f64() {
        let mut color = Rgb888::new(0, 0, 255);
        color.set_blue_normalized_f64(0.5);
        assert_eq!(color, Rgb888::new(0, 0, 128));
    }

    #[test]
    fn with_blue_rgb888_with_blue() {
        let color = Rgb888::new(0, 0, 255);
        let new_color = color.with_blue(128);
        assert_eq!(new_color, Rgb888::new(0, 0, 128));
    }
}
