use crate::rgb::macros::define_packed_argb;

define_packed_argb! {
    /// A 16-bit packed ARGB color representation.
    ///
    /// Each component is represented by 1 bit for alpha, and 5 bits each for red, green, and blue.
    ///
    /// ## Layout
    ///
    /// ```c
    /// struct Argb1555 {
    ///   uint16_t packed_argb;
    /// }
    /// ```
    ///
    /// The packed representation has the format:
    ///
    /// ```txt
    /// | 15 | 14-10 | 9-5  | 4-0  |
    /// |  A |   R   |  G   |  B   |
    /// ```
    ///
    /// ## Examples
    ///
    /// To create an `Argb1555` color from a packed representation:
    ///
    /// ```rust
    /// use gem::rgb::Argb1555;
    ///
    /// let color = Argb1555::new(0xFFFF);
    /// ```
    ///
    /// To create an `Argb1555` color from individual components:
    ///
    /// ```rust
    /// use gem::rgb::Argb1555;
    ///
    /// let color = Argb1555::from_rgb(31, 31, 31);
    /// ```
    pub struct Argb1555 {
        alpha: 1 @ 15,
        red:   5 @ 10,
        green: 5 @ 5,
        blue:  5 @ 0,
    }
}

impl Argb1555 {
    /// A fully transparent ARGB color.
    pub const TRANSPARENT: Self = Self::new(0x0000);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        alpha::HasAlpha,
        rgb::{HasBlue, HasGreen, HasRed},
    };

    #[test]
    fn test_argb1555() {
        assert_eq!(Argb1555::new(0xFFFF), Argb1555::from_rgb(31, 31, 31));
        assert_eq!(Argb1555::new(0x0000), Argb1555::TRANSPARENT);
    }

    #[test]
    fn test_argb1555_from_rgb() {
        let color = Argb1555::from_rgb(31, 31, 31);
        assert_eq!(color.alpha(), 1);
        assert_eq!(color.red(), 31);
        assert_eq!(color.green(), 31);
        assert_eq!(color.blue(), 31);
    }

    #[test]
    fn test_argb1555_transparent() {
        let color = Argb1555::TRANSPARENT;
        assert_eq!(color.alpha(), 0);
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 0);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_roundtrip() {
        let c = Argb1555::from_rgb(31, 20, 5);
        let json = serde_json::to_string(&c).unwrap();
        let back: Argb1555 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn from_argb_now_available() {
        // Previously only Argb4444 had `from_argb`; the shared macro gives
        // every packed ARGB format both constructors.
        let color = Argb1555::from_argb(0, 31, 0, 0);
        assert_eq!(color.alpha(), 0);
        assert_eq!(color.red(), 31);
    }
}
