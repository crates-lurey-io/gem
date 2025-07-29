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

    /// Returns the value of the alpha component.
    #[must_use]
    fn alpha(&self) -> C;

    /// Sets the alpha component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_alpha(&mut self, value: C);

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
    fn with_alpha_grayalpha8_with_alpha() {
        let color = GrayAlpha8::new(128, 255);
        let new_color = color.with_alpha(64);
        assert_eq!(new_color, GrayAlpha8::new(128, 64));
    }
}
