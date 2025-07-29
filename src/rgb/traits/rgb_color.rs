use crate::rgb::{HasBlue, HasGreen, HasRed};

/// A trait for types that have red, green, and blue components.
pub trait RgbColor<T>: Sized + Default + HasRed<T> + HasGreen<T> + HasBlue<T> {
    /// Creates a new color with the given red, green, and blue components.
    #[must_use]
    fn from_rgb(red: T, green: T, blue: T) -> Self {
        let mut color = Self::default();
        color.set_red(red);
        color.set_green(green);
        color.set_blue(blue);
        color
    }

    /// Returns the inner representation of the color as a tuple of red, green, and blue components.
    #[must_use]
    fn into_rgb(self) -> (T, T, T) {
        (self.red(), self.green(), self.blue())
    }
}

impl<T, C> RgbColor<C> for T where T: HasRed<C> + HasGreen<C> + HasBlue<C> + Default + Sized {}

#[cfg(test)]
mod tests {
    use crate::rgb::{HasBlue, HasGreen, HasRed};

    #[test]
    fn rgb_color_trait() {
        use crate::rgb::{Rgb888, RgbColor};

        let color: Rgb888 = RgbColor::<u8>::from_rgb(255, 0, 0);
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);

        let (r, g, b) = color.into_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }
}
