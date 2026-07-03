use crate::rgb::Bgr;

/// 8-bit BGR color representation.
///
/// Each component is represented by 8 bits, with the order being blue, green, and red,
/// stored contiguously with no padding.
///
/// ## Layout
///
/// ```c
/// struct Bgr888 {
///   uint8_t b;
///   uint8_t g;
///   uint8_t r;
/// }
/// ```
///
/// `size_of::<Bgr888>() == 3` and `align_of::<Bgr888>() == 1` — there is no
/// padding (see [`Rgb888`][crate::rgb::Rgb888] for the same note in more detail).
///
/// ## Examples
///
/// To create a `Bgr888` color from a packed representation:
///
/// ```rust
/// use gem::rgb::Bgr888;
///
/// let color = Bgr888::new(0x0000FF);
/// ```
///
/// To create a `Bgr888` color from individual components:
///
/// ```rust
/// use gem::rgb::Bgr888;
///
/// let color = Bgr888::from_bgr(255, 0, 0);
/// ```
pub type Bgr888 = Bgr<u8>;

impl Bgr888 {
    /// Creates a new BGR color from the top 24-bits of a packed ([`u32`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 23-16 | 15-8  | 7-0  |
    /// |   B   |   G   |   R  |
    /// ```
    ///
    /// Any additional bits in the packed representation are ignored.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// assert_eq!(Bgr888::new(0x0000FF), Bgr888::from_bgr(0, 0, 255));
    /// ```
    #[must_use]
    pub const fn new(packed: u32) -> Self {
        let b = ((packed >> 16) & 0xFF) as u8;
        let g = ((packed >> 8) & 0xFF) as u8;
        let r = (packed & 0xFF) as u8;
        Self::from_bgr(b, g, r)
    }
}

impl From<[u8; 3]> for Bgr888 {
    /// Creates a `Bgr888` from `[b, g, r]` — bytes in BGR (memory) order.
    fn from([b, g, r]: [u8; 3]) -> Self {
        Self::from_bgr(b, g, r)
    }
}

impl From<Bgr888> for [u8; 3] {
    /// Returns `[b, g, r]` — bytes in BGR (memory) order.
    fn from(color: Bgr888) -> Self {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        [color.blue(), color.green(), color.red()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bgr888_new() {
        assert_eq!(Bgr888::new(0x0000_00FF), Bgr888::from_bgr(0, 0, 255));
        assert_eq!(Bgr888::new(0x0000_FF00), Bgr888::from_bgr(0, 255, 0));
        assert_eq!(Bgr888::new(0x00FF_0000), Bgr888::from_bgr(255, 0, 0));
    }
}
