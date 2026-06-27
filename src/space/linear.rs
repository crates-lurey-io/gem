//! Linear (scene-referred) RGB color space.

/// A color in linear light RGB, with no gamma encoding applied.
///
/// This is the working space for most colorimetric computations: blending,
/// compositing, and conversion to perceptual spaces such as
/// [`Oklab`][crate::space::Oklab]. Convert from [`Srgb`][crate::space::Srgb]
/// before doing any light-mixing arithmetic.
///
/// # Accuracy note
///
/// When the `std` feature is **enabled**, the exact IEC 61966-2-1 formula
/// (`x^2.4`) is used for gamma decoding. Without `std`, a fast gamma-2.0
/// approximation is used instead (error up to ~10% in the midtones). Enable
/// `std` for colorimetric work; for game dev color manipulation (HSL tweaks,
/// theme generation), the approximation is usually fine.
///
/// ## Examples
///
/// ```rust
/// use gem::space::{LinearRgb, Srgb};
///
/// let srgb = Srgb::new(1.0, 0.5, 0.0);
/// let linear = LinearRgb::from(srgb);
/// let back = Srgb::from(linear);
/// // Round-trip: values should be very close to the originals.
/// assert!((back.r - srgb.r).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct LinearRgb {
    /// Linear red channel.
    pub r: f32,
    /// Linear green channel.
    pub g: f32,
    /// Linear blue channel.
    pub b: f32,
}

impl LinearRgb {
    /// Creates a new `LinearRgb` from individual channel values.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Linearly interpolates between `self` and `other` by `t`.
    ///
    /// Blending in linear light is physically correct for mixing lights.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::space::{LinearRgb, Srgb};
    ///
    /// let a = LinearRgb::from(Srgb::RED);
    /// let b = LinearRgb::from(Srgb::BLUE);
    /// let mid = a.lerp(b, 0.5);
    /// assert!((mid.r - 0.5).abs() < 1e-6);
    /// ```
    #[must_use]
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
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }
}

impl From<[f32; 3]> for LinearRgb {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self { r, g, b }
    }
}

impl From<LinearRgb> for [f32; 3] {
    fn from(c: LinearRgb) -> Self {
        [c.r, c.g, c.b]
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::space::Srgb;

    #[test]
    fn srgb_roundtrip_black_white() {
        for &srgb in &[Srgb::BLACK, Srgb::WHITE] {
            let linear = LinearRgb::from(srgb);
            let back = Srgb::from(linear);
            assert!((back.r - srgb.r).abs() < 0.01, "r mismatch: {back:?}");
            assert!((back.g - srgb.g).abs() < 0.01, "g mismatch: {back:?}");
            assert!((back.b - srgb.b).abs() < 0.01, "b mismatch: {back:?}");
        }
    }

    #[test]
    fn lerp_midpoint() {
        let a = LinearRgb::new(1.0, 0.0, 0.0);
        let b = LinearRgb::new(0.0, 0.0, 1.0);
        let mid = a.lerp(b, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-6);
        assert!((mid.b - 0.5).abs() < 1e-6);
    }

    #[test]
    fn from_array_roundtrip() {
        let arr: [f32; 3] = LinearRgb::new(0.1, 0.2, 0.3).into();
        // Same literals were stored, so exact equality holds here.
        assert!((arr[0] - 0.1).abs() < f32::EPSILON);
        assert!((arr[1] - 0.2).abs() < f32::EPSILON);
        assert!((arr[2] - 0.3).abs() < f32::EPSILON);
        let back = LinearRgb::from(arr);
        assert!((back.r - 0.1).abs() < f32::EPSILON);
    }
}
