use core::mem;

use crate::{alpha::AlphaFirst, rgb::Bgr888};

/// 8-bit ABGR color representation.
///
/// Each component is represented by 8 bits for alpha, blue, green, and red.
///
/// ## Layout
///
/// ```c
/// struct Abgr8888 {
///   uint8_t a;
///   uint8_t b;
///   uint8_t g;
///   uint8_t r;
/// }
/// ```
///
/// ## Examples
///
/// To create an `Abgr8888` color from a packed representation:
///
/// ```rust
/// use gem::rgb::Abgr8888;
///
/// let color = Abgr8888::new(0xFF0000FF);
/// ```
///
/// To create an `Abgr8888` color from individual components:
///
/// ```rust
/// use gem::rgb::Abgr8888;
///
/// let color = Abgr8888::from_abgr(255, 0, 0, 255);
/// ```
pub type Abgr8888 = AlphaFirst<u8, Bgr888>;

impl Abgr8888 {
    /// Creates a new ABGR color from the packed ([`u32`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 31-24 | 23-16 | 15-8  | 7-0  |
    /// |   A   |   B   |   G   |   R  |
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Abgr8888;
    ///
    /// assert_eq!(Abgr8888::new(0xFF0000FF), Abgr8888::from_abgr(255, 0, 0, 255));
    /// ```
    #[must_use]
    #[allow(unsafe_code)]
    pub const fn new(packed: u32) -> Self {
        unsafe { mem::transmute(packed) }
    }

    /// Creates a new ABGR color from individual component values (a, b, g, r).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::{alpha::HasAlpha, rgb::{Abgr8888, HasBlue, HasGreen, HasRed}};
    ///
    /// let color = Abgr8888::from_abgr(255, 0, 0, 255);
    /// assert_eq!(color.alpha(), 255);
    /// assert_eq!(color.blue(), 0);
    /// assert_eq!(color.green(), 0);
    /// assert_eq!(color.red(), 255);
    /// ```
    #[must_use]
    pub const fn from_abgr(a: u8, b: u8, g: u8, r: u8) -> Self {
        let packed = (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32);
        Self::new(packed)
    }
}

impl From<u32> for Abgr8888 {
    fn from(packed: u32) -> Self {
        Self::new(packed)
    }
}

#[allow(clippy::use_self)]
impl From<Abgr8888> for u32 {
    fn from(color: Abgr8888) -> Self {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        // Reconstruct the 0xAABBGGRR packed integer from components.
        (u32::from(color.alpha()) << 24)
            | (u32::from(color.blue()) << 16)
            | (u32::from(color.green()) << 8)
            | u32::from(color.red())
    }
}

impl core::fmt::LowerHex for Abgr8888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let packed: u32 = (*self).into();
        core::fmt::LowerHex::fmt(&packed, f)
    }
}

impl core::fmt::UpperHex for Abgr8888 {
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
    fn test_abgr8888_new() {
        let color = Abgr8888::new(0xFF00_00FF);
        assert_eq!(color.alpha(), 255);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.red(), 255);
    }

    #[test]
    fn test_abgr8888_from_abgr() {
        let color = Abgr8888::from_abgr(255, 0, 0, 255);
        assert_eq!(color.alpha(), 255);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.red(), 255);
    }

    #[test]
    fn from_u32() {
        assert_eq!(Abgr8888::from(0xFF00_00FF_u32), Abgr8888::from_abgr(255, 0, 0, 255));
    }

    #[test]
    fn into_u32() {
        let packed: u32 = Abgr8888::from_abgr(255, 0, 0, 255).into();
        assert_eq!(packed, 0xFF00_00FF);
    }

    #[test]
    #[cfg(feature = "std")]
    fn lower_hex() {
        // 0xAABBGGRR layout: a=0xFF, b=0x00, g=0x00, r=0xFF -> 0xFF0000FF
        assert_eq!(format!("{:08x}", Abgr8888::from_abgr(255, 0, 0, 255)), "ff0000ff");
    }
}
