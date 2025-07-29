use crate::rgb::macros;

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct Rgb565 {
    packed: u16,
}

impl Rgb565 {
    /// Creates a new RGB color from the packed ([`u16`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 15-11 | 10-5 | 4-0  |
    /// |   R   |  G   |  B   |
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb565;
    ///
    /// assert_eq!(Rgb565::new(0xFFFF), Rgb565::from_rgb(31, 63, 31));
    /// assert_eq!(Rgb565::new(0x0000), Rgb565::from_rgb(0, 0, 0));
    /// ```
    #[must_use]
    pub const fn new(packed: u16) -> Self {
        Self { packed }
    }

    /// Creates a new RGB color from individual component values (r, g, b).
    ///
    /// This is a **lossy** conversion; only the lower 5 bits of red and blue, and the lower 6 bits
    /// of green are used.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::{HasRed, HasGreen, HasBlue, Rgb565};
    ///
    /// let color = Rgb565::from_rgb(31, 63, 31);
    /// assert_eq!(color.red(), 31);
    /// assert_eq!(color.green(), 63);
    /// assert_eq!(color.blue(), 31);
    /// ```
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let packed = ((r as u16 & 0x1F) << 11) | ((g as u16 & 0x3F) << 5) | (b as u16 & 0x1F);
        Self { packed }
    }
}

macros::impl_rgb_packed!(
    Rgb565,
    red:   { shift: 11, mask: 0x1F, clear: 0xFFE0 },
    green: { shift: 5, mask: 0x3F, clear: 0xFF9F },
    blue:  { shift: 0, mask: 0x1F, clear: 0xFFE0 }
);

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
}
