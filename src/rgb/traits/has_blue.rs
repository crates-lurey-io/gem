/// A trait for types that have a 🔵 blue component.
pub trait HasBlue {
    /// The type of the blue component.
    type Component;

    /// Returns the value of the blue component.
    #[must_use]
    fn blue(&self) -> Self::Component;

    /// Sets the blue component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_blue(&mut self, value: Self::Component);

    /// Converts the color into a new type with the blue component set to the given value.
    ///
    /// The other components are left unchanged.
    #[must_use]
    fn with_blue(self, value: Self::Component) -> Self
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
    fn with_blue_rgb888_get_blue() {
        assert_eq!(Rgb888::from_rgb(0, 0, 255).blue(), 255);
    }

    #[test]
    fn with_blue_rgb888_set_blue() {
        let mut color = Rgb888::from_rgb(0, 0, 255);
        color.set_blue(128);
        assert_eq!(color, Rgb888::from_rgb(0, 0, 128));
    }

    #[test]
    fn with_blue_rgb888_with_blue() {
        let color = Rgb888::from_rgb(0, 0, 255);
        let new_color = color.with_blue(128);
        assert_eq!(new_color, Rgb888::from_rgb(0, 0, 128));
    }
}
