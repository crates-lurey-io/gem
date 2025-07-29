/// A trait for types that have a 🔴 red component.
pub trait HasRed<C> {
    /// Returns the value of the red component.
    #[must_use]
    fn red(&self) -> C;

    /// Sets the red component to the given value.
    ///
    /// If the color has other components, they are left unchanged.
    fn set_red(&mut self, value: C);

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
    fn with_red_rgb888_get_red() {
        assert_eq!(Rgb888::from_rgb(255, 0, 0).red(), 255);
    }

    #[test]
    fn with_red_rgb888_set_red() {
        let mut color = Rgb888::from_rgb(255, 0, 0);
        color.set_red(128);
        assert_eq!(color, Rgb888::from_rgb(128, 0, 0));
    }

    #[test]
    fn with_red_rgb888_with_red() {
        let color = Rgb888::from_rgb(255, 0, 0);
        let new_color = color.with_red(128);
        assert_eq!(new_color, Rgb888::from_rgb(128, 0, 0));
    }
}
