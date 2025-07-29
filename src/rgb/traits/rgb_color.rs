use crate::rgb::{HasBlue, HasGreen, HasRed};

/// A trait for types that have red, green, and blue components.
pub trait RgbColor: Sized + Default + HasRed + HasGreen + HasBlue {
    /// Creates a new color with the given red, green, and blue components.
    ///
    /// How components are represented, including possible clamping or conversion, is determined by
    /// the implementation.
    ///
    /// This method is provided as a convenience to create _any_ [`RgbColor`] type; most types will
    /// have their own dedicated constructor methods that may be more efficient and specific. For
    /// example [`Rgb888::from_rgb`][], [`Rgb565::from_rgb`][], etc.
    ///
    /// [`Rgb888::from_rgb`]: crate::rgb::Rgb888::from_rgb
    /// [`Rgb565::from_rgb`]: crate::rgb::Rgb565::from_rgb
    #[must_use]
    fn from_rgb(
        red: <Self as crate::rgb::HasRed>::Component,
        green: <Self as crate::rgb::HasGreen>::Component,
        blue: <Self as crate::rgb::HasBlue>::Component,
    ) -> Self {
        let mut color = Self::default();
        color.set_red(red);
        color.set_green(green);
        color.set_blue(blue);
        color
    }

    /// Returns the inner representation of the color as a tuple of red, green, and blue components.
    #[must_use]
    fn into_rgb(
        self,
    ) -> (
        <Self as crate::rgb::HasRed>::Component,
        <Self as crate::rgb::HasGreen>::Component,
        <Self as crate::rgb::HasBlue>::Component,
    ) {
        (self.red(), self.green(), self.blue())
    }
}

impl<T> RgbColor for T where T: HasRed + HasGreen + HasBlue + Default + Sized {}

#[cfg(test)]
mod tests {
    use crate::rgb::{HasBlue, HasGreen, HasRed};

    #[test]
    fn rgb_color_trait() {
        use crate::rgb::{Rgb888, RgbColor};

        let color: Rgb888 = RgbColor::from_rgb(255, 0, 0);
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);

        let (r, g, b) = color.into_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }
}
