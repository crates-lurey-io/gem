/// A trait for types that have a 🟢 green component.
pub trait HasGreen<C> {
    /// Returns the value of the green component.
    #[must_use]
    fn green(&self) -> C;

    /// Sets the green component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_green(&mut self, value: C);

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
    fn with_green_rgb888_get_green() {
        assert_eq!(Rgb888::from_rgb(0, 255, 0).green(), 255);
    }

    #[test]
    fn with_green_rgb888_set_green() {
        let mut color = Rgb888::from_rgb(0, 255, 0);
        color.set_green(128);
        assert_eq!(color, Rgb888::from_rgb(0, 128, 0));
    }

    #[test]
    fn with_green_rgb888_with_green() {
        let color = Rgb888::from_rgb(0, 255, 0);
        let new_color = color.with_green(128);
        assert_eq!(new_color, Rgb888::from_rgb(0, 128, 0));
    }
}
