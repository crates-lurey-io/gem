use crate::rgb::macros::define_packed_argb;

define_packed_argb! {
    /// A 16-bit packed ARGB color representation.
    ///
    /// Each component is represented by 4 bits, with the order being alpha, red, green, and blue.
    ///
    /// ## Layout
    ///
    /// ```c
    /// struct Argb4444 {
    ///   uint16_t packed_argb;
    /// }
    /// ```
    ///
    /// The packed representation has the format:
    ///
    /// ```txt
    /// | 15-12 | 11-8 | 7-4  | 3-0  |
    /// |   A   |   R  |  G   |  B   |
    /// ```
    ///
    /// ## Examples
    ///
    /// To create an `Argb4444` color from a packed representation:
    ///
    /// ```rust
    /// use gem::rgb::Argb4444;
    ///
    /// let color = Argb4444::new(0xFFFF);
    /// ```
    ///
    /// To create an `Argb4444` color from individual components:
    ///
    /// ```rust
    /// use gem::rgb::Argb4444;
    ///
    /// let color = Argb4444::from_argb(15, 15, 15, 15);
    /// ```
    pub struct Argb4444 {
        alpha: 4 @ 12,
        red:   4 @ 8,
        green: 4 @ 4,
        blue:  4 @ 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        alpha::HasAlpha,
        rgb::{HasBlue, HasGreen, HasRed},
    };

    #[test]
    fn test_argb4444() {
        assert_eq!(Argb4444::new(0xFFFF), Argb4444::from_argb(15, 15, 15, 15));
        assert_eq!(Argb4444::new(0x0000), Argb4444::from_argb(0, 0, 0, 0));
    }

    #[test]
    fn test_argb4444_from_argb() {
        let color = Argb4444::from_argb(15, 0, 0, 15);
        assert_eq!(color.alpha(), 15);
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 15);
    }

    #[test]
    fn test_argb4444_from_rgb_is_opaque() {
        let color = Argb4444::from_rgb(1, 2, 3);
        assert_eq!(color.alpha(), 15);
        assert_eq!(color.red(), 1);
        assert_eq!(color.green(), 2);
        assert_eq!(color.blue(), 3);
    }
}
