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
///   // Padding: 8 bits
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

impl From<[u8; 3]> for Rgb888 {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl From<Rgb888> for [u8; 3] {
    fn from(color: Rgb888) -> Self {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        [color.red(), color.green(), color.blue()]
    }
}

impl core::fmt::LowerHex for Rgb888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        let packed = (u32::from(self.red()) << 16)
            | (u32::from(self.green()) << 8)
            | u32::from(self.blue());
        write!(f, "{packed:06x}")
    }
}

impl core::fmt::UpperHex for Rgb888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        let packed = (u32::from(self.red()) << 16)
            | (u32::from(self.green()) << 8)
            | u32::from(self.blue());
        write!(f, "{packed:06X}")
    }
}

impl core::fmt::Display for Rgb888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{self:x}")
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

    #[test]
    fn from_array() {
        assert_eq!(Rgb888::from([255_u8, 0, 0]), Rgb888::from_rgb(255, 0, 0));
    }

    #[test]
    #[cfg(feature = "std")]
    fn into_array() {
        let arr: [u8; 3] = Rgb888::from_rgb(255, 128, 0).into();
        assert_eq!(arr, [255, 128, 0]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn lower_hex() {
        assert_eq!(format!("{:x}", Rgb888::from_rgb(255, 128, 0)), "ff8000");
    }

    #[test]
    #[cfg(feature = "std")]
    fn upper_hex() {
        assert_eq!(format!("{:X}", Rgb888::from_rgb(255, 128, 0)), "FF8000");
    }

    #[cfg(feature = "std")]
    #[test]
    fn display() {
        assert_eq!(Rgb888::from_rgb(255, 128, 0).to_string(), "#ff8000");
    }
}
