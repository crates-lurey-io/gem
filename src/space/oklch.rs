//! Oklch — cylindrical form of Oklab.

/// A color in Oklch, the cylindrical (polar) form of [`Oklab`][crate::space::Oklab].
///
/// Oklch expresses colors in terms of perceptual lightness, colorfulness (chroma),
/// and hue angle — making it the best space for intuitive perceptual color manipulation.
///
/// - `l`: lightness in `[0.0, 1.0]`.
/// - `c`: chroma (colorfulness) in `[0.0, ~0.4]` for in-gamut sRGB colors.
/// - `h`: hue in `[0.0, 1.0)` (turns, not degrees).
///
/// # Feature requirement
///
/// Converting between `Oklch` and other types requires trigonometric functions
/// (`sin`, `cos`, `atan2`). These are available when the `std` feature is enabled.
/// Without `std`, you can still construct `Oklch` values manually and call `lerp`,
/// but the `From` conversion impls are gated on `#[cfg(feature = "std")]`.
///
/// ## Examples
///
/// ```rust
/// # #[cfg(feature = "std")] {
/// use gem::space::{Oklch, Srgb};
///
/// // Rotate the hue of red by 60° (1/6 turn)
/// let red = Oklch::from(Srgb::RED);
/// let rotated = Oklch { h: (red.h + 1.0 / 6.0).rem_euclid(1.0), ..red };
/// let result = Srgb::from(rotated).clamp();
/// assert!(result.r > 0.0 || result.g > 0.0);
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Oklch {
    /// Perceived lightness in `[0.0, 1.0]`.
    pub l: f32,
    /// Chroma (colorfulness), typically `[0.0, 0.4]` for sRGB colors.
    pub c: f32,
    /// Hue in `[0.0, 1.0)` (turns, not degrees).
    pub h: f32,
}

impl Oklch {
    /// Creates a new `Oklch` color.
    ///
    /// `l` and `c` in `[0.0, 1.0]`, `h` in `[0.0, 1.0)` (turns).
    #[must_use]
    pub const fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }

    /// Returns the hue in degrees `[0.0, 360.0)`.
    #[must_use]
    pub fn hue_degrees(self) -> f32 {
        self.h * 360.0
    }

    /// Creates an `Oklch` from a hue in degrees.
    #[must_use]
    pub fn from_degrees(l: f32, c: f32, h_deg: f32) -> Self {
        Self::new(l, c, h_deg / 360.0)
    }
}

/// Interpolates in Oklch, taking the shortest path around the hue circle.
///
/// This produces perceptually uniform transitions while preserving the sense of
/// "hue rotation", which the Cartesian [`Oklab`][crate::space::Oklab] blend does
/// not: that one cuts through the desaturated middle of the color wheel.
///
/// ## Examples
///
/// ```rust
/// use gem::{Mix as _, space::Oklch};
///
/// let a = Oklch::new(0.5, 0.2, 0.0);
/// let b = Oklch::new(0.5, 0.2, 0.5);
/// let mid = a.mix(b, 0.5);
/// assert!((mid.h - 0.25).abs() < 1e-5);
/// ```
impl crate::Mix for Oklch {
    fn mix(self, other: Self, t: f32) -> Self {
        use crate::space::math::{lerp_f32, lerp_hue};
        Self {
            l: lerp_f32(self.l, other.l, t),
            c: lerp_f32(self.c, other.c, t),
            h: lerp_hue(self.h, other.h, t),
        }
    }
}

impl From<[f32; 3]> for Oklch {
    fn from([l, c, h]: [f32; 3]) -> Self {
        Self { l, c, h }
    }
}

impl From<Oklch> for [f32; 3] {
    fn from(c: Oklch) -> Self {
        [c.l, c.c, c.h]
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::Mix as _;

    #[test]
    fn mix_hue_midpoint() {
        let a = Oklch::new(0.5, 0.2, 0.0);
        let b = Oklch::new(0.5, 0.2, 0.5);
        let mid = a.mix(b, 0.5);
        assert!((mid.h - 0.25).abs() < 1e-5, "hue={}", mid.h);
    }

    #[test]
    fn mix_hue_shortest_path() {
        // 0.9 -> 0.1: shortest path wraps through 0.0
        let a = Oklch::new(0.5, 0.2, 0.9);
        let b = Oklch::new(0.5, 0.2, 0.1);
        let mid = a.mix(b, 0.5);
        assert!(mid.h < 0.05 || mid.h > 0.95, "hue={}", mid.h);
    }

    #[test]
    fn hue_degrees_roundtrip() {
        let c = Oklch::new(0.5, 0.2, 0.25);
        assert!((c.hue_degrees() - 90.0).abs() < 1e-4);
        let c2 = Oklch::from_degrees(0.5, 0.2, 90.0);
        assert!((c2.h - 0.25).abs() < 1e-4);
    }

    #[test]
    fn from_array_roundtrip() {
        let c = Oklch::new(0.5, 0.2, 0.3);
        let arr: [f32; 3] = c.into();
        assert!((arr[0] - 0.5).abs() < f32::EPSILON);
        assert!((arr[1] - 0.2).abs() < f32::EPSILON);
        assert!((arr[2] - 0.3).abs() < f32::EPSILON);
        let back = Oklch::from(arr);
        assert!((back.l - 0.5).abs() < f32::EPSILON);
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    mod conv_tests {
        use super::*;
        use crate::space::{Oklab, Srgb};

        #[test]
        fn srgb_roundtrip_red() {
            let red = Oklch::from(Srgb::RED);
            let back = Srgb::from(red).clamp();
            assert!((back.r - 1.0).abs() < 0.01, "r={}", back.r);
            assert!(back.g.abs() < 0.01, "g={}", back.g);
            assert!(back.b.abs() < 0.01, "b={}", back.b);
        }

        #[test]
        fn oklab_roundtrip() {
            let lab = Oklab::new(0.6, 0.1, -0.1);
            let lch = Oklch::from(lab);
            let back = Oklab::from(lch);
            assert!((back.l - lab.l).abs() < 1e-5);
            assert!((back.a - lab.a).abs() < 1e-5);
            assert!((back.b - lab.b).abs() < 1e-5);
        }
    }
}
