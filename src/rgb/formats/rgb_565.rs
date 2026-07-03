use crate::rgb::macros::define_packed_rgb;

define_packed_rgb! {
    /// A 16-bit packed RGB color representation.
    ///
    /// Each component is represented by 5 bits for red, 6 bits for green, and 5 bits for blue.
    ///
    /// ## Layout
    ///
    /// ```c
    /// struct Rgb565 {
    ///   uint16_t packed_rgb;
    /// }
    /// ```
    ///
    /// The packed representation has the format:
    ///
    /// ```txt
    /// | 15-11 | 10-5 | 4-0  |
    /// |   R   |  G   |  B   |
    /// ```
    ///
    /// ## Examples
    ///
    /// To create an `Rgb565` color from a packed representation:
    ///
    /// ```rust
    /// use gem::rgb::Rgb565;
    ///
    /// let color = Rgb565::new(0xFFFF);
    /// ```
    ///
    /// To create an `Rgb565` color from individual components:
    ///
    /// ```rust
    /// use gem::rgb::Rgb565;
    ///
    /// let color = Rgb565::from_rgb(31, 63, 31);
    /// ```
    ///
    /// Every packed pixel format also converts to/from
    /// [`Srgb`][crate::space::Srgb], correctly scaled for its bit depth:
    ///
    /// ```rust
    /// use gem::{rgb::Rgb565, space::Srgb};
    ///
    /// let fully_red = Rgb565::from_rgb(31, 0, 0);
    /// let srgb: Srgb = fully_red.into();
    /// assert!((srgb.r - 1.0).abs() < 1e-5); // 31/31, not 31/255
    /// ```
    pub struct Rgb565 {
        red:   5 @ 11,
        green: 6 @ 5,
        blue:  5 @ 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::rgb::{HasBlue, HasGreen, HasRed};

    use super::*;

    #[test]
    fn test_rgb565_new() {
        let color = Rgb565::new(0xFFFF);
        assert_eq!(color.red(), 31);
        assert_eq!(color.green(), 63);
        assert_eq!(color.blue(), 31);
    }

    #[test]
    fn test_rgb565_from_rgb() {
        let color = Rgb565::from_rgb(31, 63, 31);
        assert_eq!(color.red(), 31);
        assert_eq!(color.green(), 63);
        assert_eq!(color.blue(), 31);
    }

    #[test]
    fn new_from_rgb_equivalence() {
        assert_eq!(Rgb565::new(0xFFFF), Rgb565::from_rgb(31, 63, 31));
        assert_eq!(Rgb565::new(0x0000), Rgb565::from_rgb(0, 0, 0));
    }

    #[test]
    #[cfg(feature = "std")]
    fn hex_formatting() {
        assert_eq!(format!("{:04x}", Rgb565::new(0xABCD)), "abcd");
        assert_eq!(format!("{:04X}", Rgb565::new(0xABCD)), "ABCD");
    }
}
