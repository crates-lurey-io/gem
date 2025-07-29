use crate::rgb::macros;

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct Argb4444 {
    packed: u16,
}

impl Argb4444 {
    /// Creates a new ARGB color from the packed ([`u16`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 15-12 | 11-8 | 7-4  | 3-0  |
    /// |   A   |   R  |  G   |  B   |
    /// ```
    ///
    /// This is a **lossy** conversion; only the lower 4 bits of each component are used and the
    /// rest are discarded.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Argb4444;
    ///
    /// assert_eq!(Argb4444::new(0xFFFF), Argb4444::from_argb(15, 15, 15, 15));
    /// assert_eq!(Argb4444::new(0x0000), Argb4444::from_argb(0, 0, 0, 0));
    /// ```
    #[must_use]
    pub const fn new(packed: u16) -> Self {
        Self { packed }
    }

    /// Creates a new ARGB color from the individual component values (a, r, g, b).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::{alpha::HasAlpha, rgb::{Argb4444,  HasRed, HasGreen, HasBlue}};
    ///
    /// let color = Argb4444::from_argb(15, 0, 0, 15);
    /// assert_eq!(color.alpha(), 15);
    /// assert_eq!(color.red(), 0);
    /// assert_eq!(color.green(), 0);
    /// assert_eq!(color.blue(), 15);
    /// ```
    #[must_use]
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        let packed = ((a as u16 & 0x0F) << 12)
            | ((r as u16 & 0x0F) << 8)
            | ((g as u16 & 0x0F) << 4)
            | (b as u16 & 0x0F);
        Self { packed }
    }
}

macros::impl_rgb_packed!(
    Argb4444,
    red:   { shift: 8, mask: 0x0F, clear: 0xF0FF },
    green: { shift: 4, mask: 0x0F, clear: 0xFF0F },
    blue:  { shift: 0, mask: 0x0F, clear: 0xFFF0 }
);

macros::impl_with_alpha_packed!(Argb4444, 12, 0x0F, 0x0FFF);

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
}
