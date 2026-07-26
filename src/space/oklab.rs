//! Oklab perceptually-uniform color space.

/// A color in the Oklab perceptually-uniform color space.
///
/// Oklab was designed to have uniform perceptual spacing, making it ideal for:
/// - Smooth, visually-pleasing color gradients
/// - Perceptually-accurate color mixing
/// - Computing color difference
///
/// Components:
/// - `l`: lightness in `[0.0, 1.0]` — 0 is black, 1 is white.
/// - `a`: green-red axis, typically in `[-0.5, 0.5]`.
/// - `b`: blue-yellow axis, typically in `[-0.5, 0.5]`.
///
/// Reference: <https://bottosson.github.io/posts/oklab/>
///
/// ## Examples
///
/// ```rust
/// use gem::{Mix as _, space::{Oklab, Srgb}};
///
/// // Perceptually-uniform midpoint between red and blue
/// let red = Oklab::from(Srgb::RED);
/// let blue = Oklab::from(Srgb::BLUE);
/// let mid = red.mix(blue, 0.5);
/// let result = Srgb::from(mid).clamp();
/// // The result will be a perceptually balanced purple
/// assert!(result.r > 0.3 && result.b > 0.3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Oklab {
    /// Perceived lightness in `[0.0, 1.0]`.
    pub l: f32,
    /// Green-red opponent channel, typically in `[-0.5, 0.5]`.
    pub a: f32,
    /// Blue-yellow opponent channel, typically in `[-0.5, 0.5]`.
    pub b: f32,
}

impl Oklab {
    /// Creates a new `Oklab` color.
    #[must_use]
    pub const fn new(l: f32, a: f32, b: f32) -> Self {
        Self { l, a, b }
    }

    /// Returns the chroma (colorfulness), equal to `sqrt(a^2 + b^2)`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::{Oklab, Srgb};
    ///
    /// // Gray has near-zero chroma
    /// let gray = Oklab::from(Srgb::new(0.5, 0.5, 0.5));
    /// assert!(gray.chroma() < 0.01);
    /// ```
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn chroma(self) -> f32 {
        crate::space::math::sqrt(self.a * self.a + self.b * self.b)
    }

    /// Clamps lightness to `[0.0, 1.0]`.
    #[must_use]
    pub const fn clamp_lightness(self) -> Self {
        Self {
            l: self.l.clamp(0.0, 1.0),
            ..self
        }
    }
}

impl From<[f32; 3]> for Oklab {
    fn from([l, a, b]: [f32; 3]) -> Self {
        Self { l, a, b }
    }
}

impl From<Oklab> for [f32; 3] {
    fn from(c: Oklab) -> Self {
        [c.l, c.a, c.b]
    }
}

/// Interpolates in Oklab, producing perceptually uniform transitions: the
/// perceived rate of change is constant throughout the blend.
///
/// ## Examples
///
/// ```rust
/// use gem::{Mix as _, space::{Oklab, Srgb}};
///
/// let red = Oklab::from(Srgb::RED);
/// let blue = Oklab::from(Srgb::BLUE);
/// let mid = red.mix(blue, 0.5);
/// assert!(mid.l > 0.0 && mid.l < 1.0);
/// ```
impl crate::Mix for Oklab {
    fn mix(self, other: Self, t: f32) -> Self {
        use crate::space::math::lerp_f32;
        Self {
            l: lerp_f32(self.l, other.l, t),
            a: lerp_f32(self.a, other.a, t),
            b: lerp_f32(self.b, other.b, t),
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::Mix as _;
    use crate::space::Srgb;

    #[test]
    fn black_is_zero_lightness() {
        let lab = Oklab::from(Srgb::BLACK);
        assert!(lab.l.abs() < 1e-4, "l={}", lab.l);
        assert!(lab.a.abs() < 1e-4, "a={}", lab.a);
        assert!(lab.b.abs() < 1e-4, "b={}", lab.b);
    }

    #[test]
    fn white_has_max_lightness() {
        let lab = Oklab::from(Srgb::WHITE);
        assert!((lab.l - 1.0).abs() < 1e-4, "l={}", lab.l);
    }

    #[test]
    fn srgb_roundtrip() {
        for &srgb in &[Srgb::RED, Srgb::GREEN, Srgb::BLUE, Srgb::WHITE] {
            let lab = Oklab::from(srgb);
            let back = Srgb::from(lab).clamp();
            assert!(
                (back.r - srgb.r).abs() < 0.01,
                "r: {} vs {}",
                back.r,
                srgb.r
            );
            assert!(
                (back.g - srgb.g).abs() < 0.01,
                "g: {} vs {}",
                back.g,
                srgb.g
            );
            assert!(
                (back.b - srgb.b).abs() < 0.01,
                "b: {} vs {}",
                back.b,
                srgb.b
            );
        }
    }

    #[test]
    fn gray_low_chroma() {
        let gray = Oklab::from(Srgb::new(0.5, 0.5, 0.5));
        assert!(gray.chroma() < 0.01, "chroma={}", gray.chroma());
    }

    #[test]
    fn mix_midpoint_lightness() {
        let black = Oklab::from(Srgb::BLACK);
        let white = Oklab::from(Srgb::WHITE);
        let mid = black.mix(white, 0.5);
        assert!((mid.l - 0.5).abs() < 0.05, "l={}", mid.l);
    }

    #[test]
    fn from_array_roundtrip() {
        let lab = Oklab::new(0.5, 0.1, -0.1);
        let arr: [f32; 3] = lab.into();
        assert!((arr[0] - 0.5).abs() < f32::EPSILON);
        assert!((arr[1] - 0.1).abs() < f32::EPSILON);
        assert!((arr[2] - (-0.1)).abs() < f32::EPSILON);
        let back = Oklab::from(arr);
        assert!((back.l - 0.5).abs() < f32::EPSILON);
    }
}
