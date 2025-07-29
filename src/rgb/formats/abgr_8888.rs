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
}
