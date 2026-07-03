use core::mem;

use crate::{alpha::AlphaFirst, rgb::Rgb888};

/// 8-bit ARGB color representation.
///
/// Each component is represented by 8 bits for alpha, red, green, and blue.
///
/// ## Layout
///
/// ```c
/// struct Argb8888 {
///   uint8_t a;
///   uint8_t r;
///   uint8_t g;
///   uint8_t b;
/// }
/// ```
///
/// ## Examples
///
/// To create an `Argb8888` color from a packed representation:
///
/// ```rust
/// use gem::rgb::Argb8888;
///
/// let color = Argb8888::new(0xFF0000FF);
/// ```
///
/// To create an `Argb8888` color from individual components:
///
/// ```rust
/// use gem::rgb::Argb8888;
///
/// let color = Argb8888::from_argb(255, 0, 0, 255);
/// ```
pub type Argb8888 = AlphaFirst<u8, Rgb888>;

impl Argb8888 {
    /// Creates a new ARGB color from the packed ([`u32`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 31-24 | 23-16 | 15-8  | 7-0  |
    /// |   A   |   R   |   G   |   B  |
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Argb8888;
    ///
    /// assert_eq!(Argb8888::new(0xFF0000FF), Argb8888::from_argb(255, 0, 0, 255));
    /// ```
    #[must_use]
    #[allow(unsafe_code)]
    pub const fn new(packed: u32) -> Self {
        unsafe { mem::transmute(packed) }
    }

    /// Creates a new ARGB color from individual component values (a, r, g, b).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::{alpha::HasAlpha, rgb::{Argb8888, HasRed, HasGreen, HasBlue}};
    ///
    /// let color = Argb8888::from_argb(255, 0, 0, 255);
    /// assert_eq!(color.alpha(), 255);
    /// assert_eq!(color.red(), 0);
    /// assert_eq!(color.green(), 0);
    /// assert_eq!(color.blue(), 255);
    /// ```
    #[must_use]
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        let packed = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        Self::new(packed)
    }
}

impl From<u32> for Argb8888 {
    fn from(packed: u32) -> Self {
        Self::new(packed)
    }
}

#[allow(clippy::use_self)]
impl From<Argb8888> for u32 {
    fn from(color: Argb8888) -> Self {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        // Reconstruct the 0xAARRGGBB packed integer from components.
        (u32::from(color.alpha()) << 24)
            | (u32::from(color.red()) << 16)
            | (u32::from(color.green()) << 8)
            | u32::from(color.blue())
    }
}

impl core::fmt::LowerHex for Argb8888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let packed: u32 = (*self).into();
        core::fmt::LowerHex::fmt(&packed, f)
    }
}

impl core::fmt::UpperHex for Argb8888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let packed: u32 = (*self).into();
        core::fmt::UpperHex::fmt(&packed, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::rgb::{HasBlue, HasGreen, HasRed};

    use super::*;

    #[test]
    fn test_from_argb() {
        let color = Argb8888::from_argb(255, 0, 0, 255);
        assert_eq!(color.alpha(), 255);
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 255);
    }

    #[test]
    fn test_new() {
        let color = Argb8888::new(0xFF00_00FF);
        assert_eq!(color.alpha(), 255);
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 255);
    }

    #[test]
    fn from_u32() {
        assert_eq!(
            Argb8888::from(0xFF00_00FF_u32),
            Argb8888::from_argb(255, 0, 0, 255)
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn into_u32() {
        let packed: u32 = Argb8888::from_argb(255, 0, 0, 255).into();
        assert_eq!(packed, 0xFF00_00FF);
    }

    #[cfg(feature = "std")]
    #[test]
    fn lower_hex() {
        assert_eq!(
            format!("{:08x}", Argb8888::from_argb(255, 0, 0, 255)),
            "ff0000ff"
        );
    }
}
