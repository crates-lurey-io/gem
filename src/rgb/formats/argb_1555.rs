use crate::rgb::macros;

/// A 16-bit packed ARGB color representation.
///
/// Each component is represented by 1 bit for alpha, and 4 bits each for red, green, and blue.
///
/// ## Layout
///
/// ```c
/// struct Argb1555 {
///   uint16_t packed_argb;
/// }
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct Argb1555 {
    packed: u16,
}

impl Argb1555 {
    /// A fully transparent ARGB color.
    pub const TRANSPARENT: Self = Self { packed: 0x0000 };

    /// Creates a new ARGB color from the packed ([`u16`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 15 | 14-10 | 9-5  | 4-0  |
    /// |  A |   R   |  G   |  B   |
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Argb1555;
    ///
    /// assert_eq!(Argb1555::new(0xFFFF), Argb1555::from_rgb(31, 31, 31));
    /// assert_eq!(Argb1555::new(0x0000), Argb1555::TRANSPARENT);
    /// ```
    #[must_use]
    pub const fn new(packed: u16) -> Self {
        Self { packed }
    }

    /// Creates a new ARGB color from individual component values (r, g, b).
    ///
    /// The color is fully opaque (alpha = 1).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::{alpha::HasAlpha, rgb::{Argb1555, HasRed, HasGreen, HasBlue}};
    ///
    /// let color = Argb1555::from_rgb(31, 31, 31);
    /// assert_eq!(color.alpha(), 1);
    /// assert_eq!(color.red(), 31);
    /// assert_eq!(color.green(), 31);
    /// assert_eq!(color.blue(), 31);
    /// ```
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            packed: 0x8000
                | ((r as u16 & 0x1F) << 10)
                | ((g as u16 & 0x1F) << 5)
                | (b as u16 & 0x1F),
        }
    }
}

macros::impl_rgb_packed!(
    Argb1555,
    red:   { shift: 10, mask: 0x1F, clear: 0xFFE0 },
    green: { shift: 5, mask: 0x1F, clear: 0xFF9F },
    blue:  { shift: 0, mask: 0x1F, clear: 0xFFE0 }
);

macros::impl_with_alpha_packed!(Argb1555, 15, 0x01, 0x7FFF);

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
}
