use crate::rgb::Rgb;

/// 8-bit RGB color representation.
///
/// Each component is represented by 8 bits, with 8 additional bits for padding.
///
/// ## Layout
///
/// ```c
/// struct Rgb888 {
///   uint8_t r;
///   uint8_t g;
///   uint8_t b;
/// }
/// ```
///
/// ## Examples
///
/// To create an `Rgb888` color from a packed representation:
///
/// ```rust
/// use gem::rgb::Rgb888;
///
/// let color = Rgb888::new(0xFF0000);
/// ```
///
/// To create an `Rgb888` color from individual components:
///
/// ```rust
/// use gem::rgb::Rgb888;
///
/// let color = Rgb888::from_rgb(255, 0, 0);
/// ```
pub type Rgb888 = Rgb<u8>;

impl Rgb888 {
    /// Creates a new RGB color from the top 24-bits of a packed ([`u32`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 23-16 | 15-8  | 7-0  |
    /// |   R   |   G   |   B  |
    /// ```
    ///
    /// Any additional bits in the packed representation are ignored.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// assert_eq!(Rgb888::new(0xFF0000), Rgb888::from_rgb(255, 0, 0));
    /// ```
    #[must_use]
    pub const fn new(packed: u32) -> Self {
        let r = ((packed >> 16) & 0xFF) as u8;
        let g = ((packed >> 8) & 0xFF) as u8;
        let b = (packed & 0xFF) as u8;
        Self::from_rgb(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb888_new() {
        assert_eq!(Rgb888::new(0x00FF_0000), Rgb888::from_rgb(255, 0, 0));
        assert_eq!(Rgb888::new(0x0000_FF00), Rgb888::from_rgb(0, 255, 0));
        assert_eq!(Rgb888::new(0x0000_00FF), Rgb888::from_rgb(0, 0, 255));
    }
}
