/// A trait for types that have a ⚫ ⚪ gray component.
pub trait HasGray {
    /// The type of the gray component.
    type Component;

    /// Creates a new color with the given gray component.
    ///
    /// The other components are set to their default values.
    #[must_use]
    fn new_gray(value: Self::Component) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_gray(value);
        color
    }

    /// Returns the value of the gray component.
    #[must_use]
    fn gray(&self) -> Self::Component;

    /// Sets the gray component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_gray(&mut self, value: Self::Component);

    /// Converts the color into a new type with the gray component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_gray(self, value: Self::Component) -> Self
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
    fn with_gray_grayalpha8_with_gray() {
        let color = GrayAlpha8::new(255, 255);
        let new_color = color.with_gray(64);
        assert_eq!(new_color, GrayAlpha8::new(64, 255));
    }
}
