//! HSL (hue, saturation, lightness) color space.

/// A color in HSL (hue, saturation, lightness) cylindrical representation.
///
/// - `h`: hue in `[0.0, 1.0)` — a full rotation around the color wheel (0 = red,
///   1/3 = green, 2/3 = blue). Uses normalized turns rather than degrees to avoid
///   ambiguity.
/// - `s`: saturation in `[0.0, 1.0]` — 0 is gray, 1 is fully saturated.
/// - `l`: lightness in `[0.0, 1.0]` — 0 is black, 0.5 is the "pure" hue, 1 is white.
///
/// HSL is well-suited to artist-friendly color manipulation: lighten, darken,
/// saturate, complementary colors. For perceptually uniform operations, prefer
/// [`Oklch`][crate::space::Oklch].
///
/// ## Examples
///
/// ```rust
/// use gem::space::{Hsl, Srgb};
///
/// // Convert red to HSL, lighten it, convert back
/// let red_hsl = Hsl::from(Srgb::RED);
/// let lighter = red_hsl.lighten(0.1);
/// let result = Srgb::from(lighter);
/// assert!(result.r > 0.9);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Hsl {
    /// Hue in `[0.0, 1.0)` (turns, not degrees).
    pub h: f32,
    /// Saturation in `[0.0, 1.0]`.
    pub s: f32,
    /// Lightness in `[0.0, 1.0]`.
    pub l: f32,
}

impl Hsl {
    /// Creates a new `Hsl` color.
    ///
    /// `h` is in turns `[0.0, 1.0)`, `s` and `l` in `[0.0, 1.0]`.
    #[must_use]
    pub const fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }

    /// Linearly interpolates between `self` and `other` by `t`, taking the
    /// shortest path around the hue circle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsl;
    ///
    /// // Lerping hue: 0.9 (near red from the negative side) to 0.1 (near red
    /// // from the positive side) at t=0.5 should pass through 0.0 (red).
    /// let a = Hsl::new(0.9, 1.0, 0.5);
    /// let b = Hsl::new(0.1, 1.0, 0.5);
    /// let mid = a.lerp(b, 0.5);
    /// assert!(mid.h < 0.05 || mid.h > 0.95);
    /// ```
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        use crate::space::math::{lerp_f32, lerp_hue};
        Self {
            h: lerp_hue(self.h, other.h, t),
            s: lerp_f32(self.s, other.s, t),
            l: lerp_f32(self.l, other.l, t),
        }
    }

    /// Increases lightness by `amount`, clamping to `[0.0, 1.0]`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsl;
    ///
    /// let dark = Hsl::new(0.0, 1.0, 0.3);
    /// let lighter = dark.lighten(0.2);
    /// assert!((lighter.l - 0.5).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn lighten(self, amount: f32) -> Self {
        Self {
            l: (self.l + amount).clamp(0.0, 1.0),
            ..self
        }
    }

    /// Decreases lightness by `amount`, clamping to `[0.0, 1.0]`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsl;
    ///
    /// let bright = Hsl::new(0.0, 1.0, 0.8);
    /// let darker = bright.darken(0.2);
    /// assert!((darker.l - 0.6).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn darken(self, amount: f32) -> Self {
        Self {
            l: (self.l - amount).clamp(0.0, 1.0),
            ..self
        }
    }

    /// Increases saturation by `amount`, clamping to `[0.0, 1.0]`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsl;
    ///
    /// let muted = Hsl::new(0.0, 0.3, 0.5);
    /// let vivid = muted.saturate(0.3);
    /// assert!((vivid.s - 0.6).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn saturate(self, amount: f32) -> Self {
        Self {
            s: (self.s + amount).clamp(0.0, 1.0),
            ..self
        }
    }

    /// Decreases saturation by `amount`, clamping to `[0.0, 1.0]`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsl;
    ///
    /// let vivid = Hsl::new(0.0, 1.0, 0.5);
    /// let muted = vivid.desaturate(0.4);
    /// assert!((muted.s - 0.6).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn desaturate(self, amount: f32) -> Self {
        Self {
            s: (self.s - amount).clamp(0.0, 1.0),
            ..self
        }
    }

    /// Returns the complementary color (hue shifted by 180°, i.e., +0.5 turns).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::Hsl;
    ///
    /// let red = Hsl::new(0.0, 1.0, 0.5);
    /// let cyan = red.complement();
    /// assert!((cyan.h - 0.5).abs() < 1e-6);
    /// ```
    #[must_use]
    pub fn complement(self) -> Self {
        use crate::space::math::rem_euclid;
        Self {
            h: rem_euclid(self.h + 0.5, 1.0),
            ..self
        }
    }

    /// Returns the hue in degrees `[0.0, 360.0)`.
    ///
    /// Convenience for interop with tools that expect degrees.
    #[must_use]
    pub fn hue_degrees(self) -> f32 {
        self.h * 360.0
    }

    /// Creates an `Hsl` from a hue in degrees `[0, 360)`.
    #[must_use]
    pub fn from_degrees(h_deg: f32, s: f32, l: f32) -> Self {
        Self::new(h_deg / 360.0, s, l)
    }
}

impl From<[f32; 3]> for Hsl {
    fn from([h, s, l]: [f32; 3]) -> Self {
        Self { h, s, l }
    }
}

impl From<Hsl> for [f32; 3] {
    fn from(c: Hsl) -> Self {
        [c.h, c.s, c.l]
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::space::Srgb;

    #[test]
    fn red_to_hsl() {
        let hsl = Hsl::from(Srgb::RED);
        assert!(hsl.h.abs() < 1e-4 || (hsl.h - 1.0).abs() < 1e-4, "hue={}", hsl.h);
        assert!((hsl.s - 1.0).abs() < 1e-4, "sat={}", hsl.s);
        assert!((hsl.l - 0.5).abs() < 1e-4, "lit={}", hsl.l);
    }

    #[test]
    fn green_to_hsl() {
        let hsl = Hsl::from(Srgb::GREEN);
        assert!((hsl.h - 1.0 / 3.0).abs() < 1e-4, "hue={}", hsl.h);
        assert!((hsl.s - 1.0).abs() < 1e-4, "sat={}", hsl.s);
        assert!((hsl.l - 0.5).abs() < 1e-4, "lit={}", hsl.l);
    }

    #[test]
    fn hsl_to_srgb_roundtrip() {
        let original = Srgb::new(0.8, 0.3, 0.5);
        let hsl = Hsl::from(original);
        let back = Srgb::from(hsl);
        assert!((back.r - original.r).abs() < 1e-5, "r: {} vs {}", back.r, original.r);
        assert!((back.g - original.g).abs() < 1e-5, "g: {} vs {}", back.g, original.g);
        assert!((back.b - original.b).abs() < 1e-5, "b: {} vs {}", back.b, original.b);
    }

    #[test]
    fn gray_has_zero_saturation() {
        let gray = Srgb::new(0.5, 0.5, 0.5);
        let hsl = Hsl::from(gray);
        assert!(hsl.s < 1e-5, "saturation={}", hsl.s);
        assert!((hsl.l - 0.5).abs() < 1e-5, "lightness={}", hsl.l);
    }

    #[test]
    fn lighten() {
        let hsl = Hsl::new(0.0, 1.0, 0.3).lighten(0.2);
        assert!((hsl.l - 0.5).abs() < 1e-6);
    }

    #[test]
    fn darken() {
        let hsl = Hsl::new(0.0, 1.0, 0.8).darken(0.2);
        assert!((hsl.l - 0.6).abs() < 1e-6);
    }

    #[test]
    fn saturate() {
        let hsl = Hsl::new(0.0, 0.3, 0.5).saturate(0.3);
        assert!((hsl.s - 0.6).abs() < 1e-6);
    }

    #[test]
    fn desaturate() {
        let hsl = Hsl::new(0.0, 1.0, 0.5).desaturate(0.4);
        assert!((hsl.s - 0.6).abs() < 1e-6);
    }

    #[test]
    fn lighten_clamps() {
        let hsl = Hsl::new(0.0, 1.0, 0.9).lighten(0.5);
        assert_eq!(hsl.l, 1.0);
    }

    #[test]
    fn complement_red_is_cyan() {
        let red = Hsl::new(0.0, 1.0, 0.5);
        let cyan = red.complement();
        assert!((cyan.h - 0.5).abs() < 1e-6);
    }

    #[test]
    fn complement_wraps() {
        let hsl = Hsl::new(0.7, 1.0, 0.5);
        let comp = hsl.complement();
        assert!((comp.h - 0.2).abs() < 1e-6);
    }

    #[test]
    fn hue_degrees_roundtrip() {
        let hsl = Hsl::new(0.25, 1.0, 0.5);
        assert!((hsl.hue_degrees() - 90.0).abs() < 1e-4);

        let hsl2 = Hsl::from_degrees(90.0, 1.0, 0.5);
        assert!((hsl2.h - 0.25).abs() < 1e-4);
    }

    #[test]
    fn lerp_hue_shortest_path() {
        let a = Hsl::new(0.9, 1.0, 0.5);
        let b = Hsl::new(0.1, 1.0, 0.5);
        let mid = a.lerp(b, 0.5);
        // Shortest path from 0.9 to 0.1 goes through 0.0
        assert!(mid.h < 0.05 || mid.h > 0.95, "hue={}", mid.h);
    }
}
