use crate::scalar::{
    intensity::Intensity,
    normalize::{NormF32, NormF64, Normalize},
};

/// A trait for types that have a 🔴 red component.
pub trait WithRed<C> {
    /// Creates a new color with the given red component.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_red(value: C) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_red(value);
        color
    }

    /// Creates a new color with the red component set to the given normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_red_normalized_f32(value: f32) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f32>,
    {
        let mut color = Self::default();
        color.set_red_normalized_f32(value);
        color
    }

    /// Creates a new color with the red component set to the given normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_red_normalized_f64(value: f64) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f64>,
    {
        let mut color = Self::default();
        color.set_red_normalized_f64(value);
        color
    }

    /// Returns the value of the red component.
    #[must_use]
    fn red(&self) -> C;

    /// Sets the red component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_red(&mut self, value: C);

    /// Returns the red intensity of the color, as a normalized 32-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn red_normalized_f32(&self) -> f32
    where
        C: Normalize<NormF32>,
    {
        self.red().normalize_clamped().into_inner()
    }

    /// Returns the red intensity of the color, as a normalized 64-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn red_normalized_f64(&self) -> f64
    where
        C: Normalize<NormF64>,
    {
        self.red().normalize_clamped().into_inner()
    }

    /// Sets the red component based on the normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    fn set_red_normalized_f32(&mut self, value: f32)
    where
        C: Intensity + Into<f32>,
    {
        self.set_red(C::from_normalized_f32(value));
    }

    /// Sets the red component based on the normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    fn set_red_normalized_f64(&mut self, value: f64)
    where
        C: Intensity + Into<f64>,
    {
        self.set_red(C::from_normalized_f64(value));
    }

    /// Converts the color into a new type with the red component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_red(self, value: C) -> Self
    where
        Self: Copy + Clone,
    {
        let mut color = self;
        color.set_red(value);
        color
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::rgb::Rgb888;

    use super::*;

    #[test]
    fn with_red_rgb888_new_red() {
        assert_eq!(Rgb888::new_red(255), Rgb888::new(255, 0, 0));
    }

    #[test]
    fn with_red_rgb888_new_red_normalized_f32() {
        assert_eq!(Rgb888::new_red_normalized_f32(1.0), Rgb888::new(255, 0, 0));
    }

    #[test]
    fn with_red_rgb888_new_red_normalized_f64() {
        assert_eq!(Rgb888::new_red_normalized_f64(1.0), Rgb888::new(255, 0, 0));
    }

    #[test]
    fn with_red_rgb888_get_red() {
        assert_eq!(Rgb888::new(255, 0, 0).red(), 255);
    }

    #[test]
    fn with_red_rgb888_set_red() {
        let mut color = Rgb888::new(255, 0, 0);
        color.set_red(128);
        assert_eq!(color, Rgb888::new(128, 0, 0));
    }

    #[test]
    fn with_red_rgb888_get_red_normalized_f32() {
        let color = Rgb888::new(255, 0, 0);
        assert_eq!(color.red_normalized_f32(), 1.0);
    }

    #[test]
    fn with_red_rgb888_set_red_normalized_f32() {
        let mut color = Rgb888::new(255, 0, 0);
        color.set_red_normalized_f32(0.5);
        assert_eq!(color, Rgb888::new(128, 0, 0));
    }

    #[test]
    fn with_red_rgb888_get_red_normalized_f64() {
        let color = Rgb888::new(255, 0, 0);
        assert_eq!(color.red_normalized_f64(), 1.0);
    }

    #[test]
    fn with_red_rgb888_set_red_normalized_f64() {
        let mut color = Rgb888::new(255, 0, 0);
        color.set_red_normalized_f64(0.5);
        assert_eq!(color, Rgb888::new(128, 0, 0));
    }

    #[test]
    fn with_red_rgb888_with_red() {
        let color = Rgb888::new(255, 0, 0);
        let new_color = color.with_red(128);
        assert_eq!(new_color, Rgb888::new(128, 0, 0));
    }
}
