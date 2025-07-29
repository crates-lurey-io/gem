use crate::{
    alpha::HasAlpha,
    rgb::{HasBlue, HasGreen, HasRed},
};

/// A trait for types that have red, green, blue, and alpha components.
pub trait RgbaColor: Sized + Default + HasRed + HasGreen + HasBlue + HasAlpha {
    /// Creates a new color with the given red, green, blue, and alpha components.
    ///
    /// How components are represented, including possible clamping or conversion, is determined by
    /// the implementation.
    ///
    /// This method is provided as a convenience to create _any_ [`RgbaColor`] type; most types will
    /// have their own dedicated constructor methods that may be more efficient and specific. For
    /// example [`Abgr8888::from_rgba`][], [`Rgbaf32::from_rgba`][], etc.
    ///
    /// [`Abgr8888::from_rgba`]: crate::rgb::Abgr8888::from_rgba
    /// [`Rgbaf32::from_rgba`]: crate::rgb::Rgbaf32::from_rgba
    #[must_use]
    fn from_rgba(
        red: <Self as crate::rgb::HasRed>::Component,
        green: <Self as crate::rgb::HasGreen>::Component,
        blue: <Self as crate::rgb::HasBlue>::Component,
        alpha: <Self as crate::alpha::HasAlpha>::Component,
    ) -> Self {
        let mut color = Self::default();
        color.set_red(red);
        color.set_green(green);
        color.set_blue(blue);
        color.set_alpha(alpha);
        color
    }

    /// Returns the inner representation of the color as a tuple of red, green, blue, and alpha components.
    #[must_use]
    fn into_rgba(
        self,
    ) -> (
        <Self as crate::rgb::HasRed>::Component,
        <Self as crate::rgb::HasGreen>::Component,
        <Self as crate::rgb::HasBlue>::Component,
        <Self as crate::alpha::HasAlpha>::Component,
    ) {
        (self.red(), self.green(), self.blue(), self.alpha())
    }
}

impl<T> RgbaColor for T where T: HasRed + HasGreen + HasBlue + HasAlpha + Default {}

#[cfg(test)]
mod tests {
    use crate::rgb::{Abgr8888, HasBlue, HasGreen, HasRed, RgbaColor};

    #[test]
    fn rgba_color_trait() {
        let color: Abgr8888 = RgbaColor::from_rgba(255, 0, 0, 255);
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
