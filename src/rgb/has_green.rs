use crate::scalar::{
    intensity::Intensity,
    normalize::{NormF32, NormF64, Normalize},
};

/// A trait for types that have a 🟢 green component.
pub trait HasGreen<C> {
    /// Creates a new color with the given green component.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_green(value: C) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_green(value);
        color
    }

    /// Creates a new color with the green component set to the given normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn new_green_normalized_f32(value: f32) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f32>,
    {
        Self::new_green(C::from_normalized_f32(value))
    }

    /// Creates a new color with the green component set to the given normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    ///
    /// The other components are set to their default values.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn new_green_normalized_f64(value: f64) -> Self
    where
        Self: Sized + Default,
        C: Intensity + Into<f64>,
    {
        Self::new_green(C::from_normalized_f64(value))
    }

    /// Returns the value of the green component.
    #[must_use]
    fn green(&self) -> C;

    /// Sets the green component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_green(&mut self, value: C);

    /// Returns the green intensity of the color, as a normalized 32-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn green_normalized_f32(&self) -> f32
    where
        C: Normalize<NormF32>,
    {
        self.green().normalize_clamped().into_inner()
    }

    /// Returns the green intensity of the color, as a normalized 64-bit floating point value.
    ///
    /// This value is in the range [0.0, 1.0].
    #[must_use]
    fn green_normalized_f64(&self) -> f64
    where
        C: Normalize<NormF64>,
    {
        self.green().normalize_clamped().into_inner()
    }

    /// Sets the green component based on the normalized 32-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn set_green_normalized_f32(&mut self, value: f32)
    where
        C: Intensity + Into<f32>,
    {
        self.set_green(C::from_normalized_f32(value));
    }

    /// Sets the green component based on the normalized 64-bit floating point value.
    ///
    /// The number is clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    fn set_green_normalized_f64(&mut self, value: f64)
    where
        C: Intensity + Into<f64>,
    {
        self.set_green(C::from_normalized_f64(value));
    }

    /// Converts the color into a new type with the green component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_green(self, value: C) -> Self
    where
        Self: Copy + Clone,
    {
        let mut color = self;
        color.set_green(value);
        color
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::rgb::Rgb888;

    use super::*;

    #[test]
    fn with_green_rgb888_new_green() {
        assert_eq!(Rgb888::new_green(255), Rgb888::new(0, 255, 0));
    }

    #[test]
    fn with_green_rgb888_new_green_normalized_f32() {
        assert_eq!(
            Rgb888::new_green_normalized_f32(1.0),
            Rgb888::new(0, 255, 0)
        );
    }

    #[test]
    fn with_green_rgb888_new_green_normalized_f64() {
        assert_eq!(
            Rgb888::new_green_normalized_f64(1.0),
            Rgb888::new(0, 255, 0)
        );
    }

    #[test]
    fn with_green_rgb888_get_green() {
        assert_eq!(Rgb888::new(0, 255, 0).green(), 255);
    }

    #[test]
    fn with_green_rgb888_set_green() {
        let mut color = Rgb888::new(0, 255, 0);
        color.set_green(128);
        assert_eq!(color, Rgb888::new(0, 128, 0));
    }

    #[test]
    fn with_green_rgb888_get_green_normalized_f32() {
        let color = Rgb888::new(0, 255, 0);
        assert_eq!(color.green_normalized_f32(), 1.0);
    }

    #[test]
    fn with_green_rgb888_set_green_normalized_f32() {
        let mut color = Rgb888::new(0, 255, 0);
        color.set_green_normalized_f32(0.5);
        assert_eq!(color, Rgb888::new(0, 128, 0));
    }

    #[test]
    fn with_green_rgb888_get_green_normalized_f64() {
        let color = Rgb888::new(0, 255, 0);
        assert_eq!(color.green_normalized_f64(), 1.0);
    }

    #[test]
    fn with_green_rgb888_set_green_normalized_f64() {
        let mut color = Rgb888::new(0, 255, 0);
        color.set_green_normalized_f64(0.5);
        assert_eq!(color, Rgb888::new(0, 128, 0));
    }

    #[test]
    fn with_green_rgb888_with_green() {
        let color = Rgb888::new(0, 255, 0);
        let new_color = color.with_green(128);
        assert_eq!(new_color, Rgb888::new(0, 128, 0));
    }
}
