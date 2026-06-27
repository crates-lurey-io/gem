//! HSV (hue, saturation, value) color space.

/// A color in HSV (hue, saturation, value) cylindrical representation.
///
/// - `h`: hue in `[0.0, 1.0)` — turns around the color wheel (0 = red, 1/3 = green, 2/3 = blue).
/// - `s`: saturation in `[0.0, 1.0]` — 0 is white, 1 is fully saturated.
/// - `v`: value (brightness) in `[0.0, 1.0]` — 0 is black, 1 is maximum brightness.
///
/// HSV is common in color pickers and artistic workflows. Note that unlike
/// [`Hsl`][crate::space::Hsl], decreasing `s` in HSV moves toward white (not gray),
/// while decreasing `v` moves toward black.
///
/// ## Examples
///
/// ```rust
/// use gem::space::{Hsv, Srgb};
///
/// let red = Hsv::from(Srgb::RED);
/// assert!((red.v - 1.0).abs() < 1e-5);
/// assert!((red.s - 1.0).abs() < 1e-5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Hsv {
    /// Hue in `[0.0, 1.0)` (turns, not degrees).
    pub h: f32,
    /// Saturation in `[0.0, 1.0]`.
    pub s: f32,
    /// Value (brightness) in `[0.0, 1.0]`.
    pub v: f32,
}

impl Hsv {
    /// Creates a new `Hsv` color.
    ///
    /// `h` is in turns `[0.0, 1.0)`, `s` and `v` in `[0.0, 1.0]`.
    #[must_use]
    pub const fn new(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v }
    }

    /// Linearly interpolates between `self` and `other` by `t`, taking the
    /// shortest path around the hue circle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsv;
    ///
    /// let a = Hsv::new(0.0, 1.0, 1.0);
    /// let b = Hsv::new(1.0 / 3.0, 1.0, 1.0);
    /// let mid = a.lerp(b, 0.5);
    /// assert!((mid.h - 1.0 / 6.0).abs() < 1e-5);
    /// ```
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        use crate::space::math::{lerp_f32, lerp_hue};
        Self {
            h: lerp_hue(self.h, other.h, t),
            s: lerp_f32(self.s, other.s, t),
            v: lerp_f32(self.v, other.v, t),
        }
    }

    /// Returns the hue in degrees `[0.0, 360.0)`.
    #[must_use]
    pub fn hue_degrees(self) -> f32 {
        self.h * 360.0
    }

    /// Creates an `Hsv` from a hue in degrees.
    #[must_use]
    pub fn from_degrees(h_deg: f32, s: f32, v: f32) -> Self {
        Self::new(h_deg / 360.0, s, v)
    }
}

impl From<[f32; 3]> for Hsv {
    fn from([h, s, v]: [f32; 3]) -> Self {
        Self { h, s, v }
    }
}

impl From<Hsv> for [f32; 3] {
    fn from(c: Hsv) -> Self {
        [c.h, c.s, c.v]
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::space::Srgb;

    #[test]
    fn red_to_hsv() {
        let hsv = Hsv::from(Srgb::RED);
        assert!(hsv.h.abs() < 1e-4 || (hsv.h - 1.0).abs() < 1e-4);
        assert!((hsv.s - 1.0).abs() < 1e-4);
        assert!((hsv.v - 1.0).abs() < 1e-4);
    }

    #[test]
    fn black_has_zero_value() {
        let hsv = Hsv::from(Srgb::BLACK);
        assert!(hsv.v.abs() < 1e-5);
    }

    #[test]
    fn white_has_zero_saturation() {
        let hsv = Hsv::from(Srgb::WHITE);
        assert!(hsv.s.abs() < 1e-5);
        assert!((hsv.v - 1.0).abs() < 1e-5);
    }

    #[test]
    fn roundtrip() {
        let original = Srgb::new(0.8, 0.3, 0.5);
        let hsv = Hsv::from(original);
        let back = Srgb::from(hsv);
        assert!((back.r - original.r).abs() < 1e-5);
        assert!((back.g - original.g).abs() < 1e-5);
        assert!((back.b - original.b).abs() < 1e-5);
    }

    #[test]
    fn lerp_hue() {
        let a = Hsv::new(0.0, 1.0, 1.0);
        let b = Hsv::new(1.0 / 3.0, 1.0, 1.0);
        let mid = a.lerp(b, 0.5);
        assert!((mid.h - 1.0 / 6.0).abs() < 1e-5, "hue={}", mid.h);
    }
}
