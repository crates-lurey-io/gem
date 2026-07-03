//! Gamma-corrected sRGB color space.

/// A color in the sRGB color space (IEC 61966-2-1).
///
/// Components are gamma-corrected and stored in the `[0.0, 1.0]` range.
/// This is the color space used by CSS, PNG, JPEG, and most everyday images.
///
/// To perform perceptually-uniform operations such as blending and gradients,
/// convert to [`Oklab`][crate::space::Oklab] or [`LinearRgb`][crate::space::LinearRgb] first.
///
/// ## Examples
///
/// ```rust
/// use gem::space::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let purple = red.lerp(blue, 0.5);
/// assert!((purple.r - 0.5).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Srgb {
    /// Red channel in `[0.0, 1.0]`.
    pub r: f32,
    /// Green channel in `[0.0, 1.0]`.
    pub g: f32,
    /// Blue channel in `[0.0, 1.0]`.
    pub b: f32,
}

impl Srgb {
    /// Black: `(0, 0, 0)`.
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    /// White: `(1, 1, 1)`.
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    /// Opaque red: `(1, 0, 0)`.
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    /// Opaque green: `(0, 1, 0)`.
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    /// Opaque blue: `(0, 0, 1)`.
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);

    /// Creates a new `Srgb` color from individual channel values.
    ///
    /// Values outside `[0.0, 1.0]` are stored as-is; call [`clamp`][Self::clamp]
    /// before converting to integer pixel formats.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Creates an `Srgb` color from an `0xRRGGBB` packed integer.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Srgb;
    ///
    /// let orange = Srgb::from_u32(0xFF8000);
    /// assert!((orange.r - 1.0).abs() < 1e-3);
    /// assert!((orange.g - 0.502).abs() < 1e-2);
    /// assert_eq!(orange.b, 0.0);
    /// ```
    #[must_use]
    pub fn from_u32(packed: u32) -> Self {
        // Mask to u8 range first so the cast to f32 is exact (u8 is 8 bits,
        // well within f32's 23-bit mantissa).
        let r = f32::from(((packed >> 16) & 0xFF) as u8) / 255.0;
        let g = f32::from(((packed >> 8) & 0xFF) as u8) / 255.0;
        let b = f32::from((packed & 0xFF) as u8) / 255.0;
        Self { r, g, b }
    }

    /// Returns the color as an `0xRRGGBB` packed integer.
    ///
    /// Channels are clamped to `[0.0, 1.0]` before packing.
    #[must_use]
    pub fn to_u32(self) -> u32 {
        use crate::space::math::channel_to_u8;
        let r = u32::from(channel_to_u8(self.r));
        let g = u32::from(channel_to_u8(self.g));
        let b = u32::from(channel_to_u8(self.b));
        (r << 16) | (g << 8) | b
    }

    /// Parses a CSS hex color string.
    ///
    /// Accepts `#rgb`, `#rrggbb` (case-insensitive). Returns `None` for
    /// unrecognized input.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Srgb;
    ///
    /// let red = Srgb::from_hex("#ff0000").unwrap();
    /// assert!((red.r - 1.0).abs() < 1e-6);
    ///
    /// let short = Srgb::from_hex("#f00").unwrap();
    /// assert!((short.r - 1.0).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#')?;
        match s.len() {
            3 => {
                let r = u8::from_str_radix(&s[0..1], 16).ok()?;
                let g = u8::from_str_radix(&s[1..2], 16).ok()?;
                let b = u8::from_str_radix(&s[2..3], 16).ok()?;
                // Expand nibble to byte: 0xF -> 0xFF
                Some(Self {
                    r: f32::from(r | (r << 4)) / 255.0,
                    g: f32::from(g | (g << 4)) / 255.0,
                    b: f32::from(b | (b << 4)) / 255.0,
                })
            }
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                Some(Self {
                    r: f32::from(r) / 255.0,
                    g: f32::from(g) / 255.0,
                    b: f32::from(b) / 255.0,
                })
            }
            _ => None,
        }
    }

    /// Linearly interpolates between `self` and `other` by `t`.
    ///
    /// Note: for perceptually uniform interpolation, convert to
    /// [`Oklch`][crate::space::Oklch] first.
    ///
    /// `t = 0.0` returns `self`; `t = 1.0` returns `other`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Srgb;
    ///
    /// let mid = Srgb::RED.lerp(Srgb::BLUE, 0.5);
    /// assert!((mid.r - 0.5).abs() < 1e-6);
    /// assert!((mid.b - 0.5).abs() < 1e-6);
    /// ```
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        use crate::space::math::lerp_f32;
        Self {
            r: lerp_f32(self.r, other.r, t),
            g: lerp_f32(self.g, other.g, t),
            b: lerp_f32(self.b, other.b, t),
        }
    }

    /// Clamps all channels to `[0.0, 1.0]`.
    #[must_use]
    pub const fn clamp(self) -> Self {
        use crate::space::math::clamp;
        Self {
            r: clamp(self.r, 0.0, 1.0),
            g: clamp(self.g, 0.0, 1.0),
            b: clamp(self.b, 0.0, 1.0),
        }
    }

    /// Returns the relative luminance of this color (per WCAG 2.x / ITU-R BT.709).
    ///
    /// The result is in `[0.0, 1.0]` where 0 is black and 1 is white.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Srgb;
    ///
    /// assert!((Srgb::WHITE.luminance() - 1.0).abs() < 1e-4);
    /// assert!(Srgb::BLACK.luminance() < 1e-6);
    /// ```
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn luminance(self) -> f32 {
        use crate::space::math::srgb_to_linear_channel as lin;
        0.2126 * lin(self.r) + 0.7152 * lin(self.g) + 0.0722 * lin(self.b)
    }

    /// Returns whether this color is considered "dark" (luminance < 0.5).
    #[must_use]
    pub fn is_dark(self) -> bool {
        self.luminance() < 0.5
    }
}

impl From<[f32; 3]> for Srgb {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self { r, g, b }
    }
}

impl From<Srgb> for [f32; 3] {
    fn from(c: Srgb) -> Self {
        [c.r, c.g, c.b]
    }
}

/// Formats as `#rrggbb` (lowercase hex).
impl core::fmt::Display for Srgb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:06x}", self.to_u32())
    }
}

/// Formats as `rrggbb` lowercase hex.
impl core::fmt::LowerHex for Srgb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.to_u32(), f)
    }
}

/// Formats as `RRGGBB` uppercase hex.
impl core::fmt::UpperHex for Srgb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.to_u32(), f)
    }
}

#[cfg(test)]
#[allow(
    clippy::float_cmp,        // Exact float equality is intentional in unit tests
    clippy::float_cmp_const,  // Comparing against named constants is fine
)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Srgb::new(1.0, 0.5, 0.0);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.5);
        assert_eq!(c.b, 0.0);
    }

    #[test]
    fn from_u32_roundtrip() {
        let c = Srgb::from_u32(0xFF_8000);
        assert_eq!(c.to_u32(), 0xFF_8000);
    }

    #[test]
    fn from_hex_six_digit() {
        let c = Srgb::from_hex("#ff0000").unwrap();
        assert!((c.r - 1.0).abs() < 1e-6, "r={}", c.r);
        assert!(c.g.abs() < 1e-6);
        assert!(c.b.abs() < 1e-6);
    }

    #[test]
    fn from_hex_three_digit() {
        let a = Srgb::from_hex("#f00").unwrap();
        let b = Srgb::from_hex("#ff0000").unwrap();
        assert!((a.r - b.r).abs() < 1e-6);
        assert!((a.g - b.g).abs() < 1e-6);
        assert!((a.b - b.b).abs() < 1e-6);
    }

    #[test]
    fn from_hex_uppercase() {
        assert!(Srgb::from_hex("#FF0000").is_some());
    }

    #[test]
    fn from_hex_invalid() {
        assert!(Srgb::from_hex("ff0000").is_none()); // no #
        assert!(Srgb::from_hex("#gg0000").is_none()); // invalid hex
        assert!(Srgb::from_hex("#ffff").is_none()); // wrong length
    }

    #[test]
    fn lerp_midpoint() {
        let mid = Srgb::RED.lerp(Srgb::BLUE, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-6);
        assert!((mid.g).abs() < 1e-6);
        assert!((mid.b - 0.5).abs() < 1e-6);
    }

    #[test]
    fn lerp_endpoints() {
        assert_eq!(Srgb::RED.lerp(Srgb::BLUE, 0.0), Srgb::RED);
        assert_eq!(Srgb::RED.lerp(Srgb::BLUE, 1.0), Srgb::BLUE);
    }

    #[test]
    #[cfg(feature = "std")]
    fn clamp_out_of_range() {
        let c = Srgb::new(2.0, -1.0, 0.5).clamp();
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.5);
    }

    #[test]
    #[cfg(feature = "std")]
    fn display() {
        assert_eq!(Srgb::RED.to_string(), "#ff0000");
        assert_eq!(Srgb::BLACK.to_string(), "#000000");
        assert_eq!(Srgb::WHITE.to_string(), "#ffffff");
    }

    #[test]
    fn from_array_roundtrip() {
        let arr: [f32; 3] = Srgb::RED.into();
        assert_eq!(arr, [1.0, 0.0, 0.0]);
        assert_eq!(Srgb::from(arr), Srgb::RED);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_roundtrip() {
        let c = Srgb::new(0.25, 0.5, 0.75);
        let json = serde_json::to_string(&c).unwrap();
        let back: Srgb = serde_json::from_str(&json).unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn luminance_white_black() {
        assert!((Srgb::WHITE.luminance() - 1.0).abs() < 1e-3);
        assert!(Srgb::BLACK.luminance() < 1e-6);
    }

    #[test]
    fn is_dark() {
        assert!(Srgb::BLACK.is_dark());
        assert!(!Srgb::WHITE.is_dark());
    }
}
