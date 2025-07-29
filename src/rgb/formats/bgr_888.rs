use crate::rgb::Bgr;

/// 8-bit BGR color representation.
///
/// Each component is represented by 8 bits, with the order being blue, green, and red.
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
