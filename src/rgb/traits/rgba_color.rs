use crate::{
    alpha::HasAlpha,
    rgb::{HasBlue, HasGreen, HasRed},
};

/// A trait for types that have red, green, blue, and alpha components.
pub trait RgbaColor<T>:
    Sized + Default + HasRed<T> + HasGreen<T> + HasBlue<T> + HasAlpha<T>
{
    /// Creates a new color with the given red, green, blue, and alpha components.
    #[must_use]
    fn from_rgba(red: T, green: T, blue: T, alpha: T) -> Self {
        let mut color = Self::default();
        color.set_red(red);
        color.set_green(green);
        color.set_blue(blue);
        color.set_alpha(alpha);
        color
    }

    /// Returns the inner representation of the color as a tuple of red, green, blue, and alpha components.
    #[must_use]
    fn into_rgba(self) -> (T, T, T, T) {
        (self.red(), self.green(), self.blue(), self.alpha())
    }
}

impl<T, C> RgbaColor<C> for T where T: HasRed<C> + HasGreen<C> + HasBlue<C> + HasAlpha<C> + Default {}

#[cfg(test)]
mod tests {
    use crate::rgb::{Abgr8888, HasBlue, HasGreen, HasRed, RgbaColor};

    #[test]
    fn rgba_color_trait() {
        let color: Abgr8888 = RgbaColor::<u8>::from_rgba(255, 0, 0, 255);
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.alpha(), 255);

        let (r, g, b, a) = color.into_rgba();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert_eq!(a, 255);
    }
}
