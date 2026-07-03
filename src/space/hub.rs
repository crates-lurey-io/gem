//! A hub-and-spoke conversion trait for color spaces.
//!
//! [`convert.rs`][super::convert] hand-writes a `From` impl for every pair of
//! spaces that needs to convert directly. That's fine for the five spaces
//! this crate ships today, but it's an O(n\u00b2) maintenance burden: adding a
//! sixth space (say, CIE Xyz) would mean writing five new `From` impls just
//! to reach the existing ones, and *that* count keeps growing with every
//! space added after it.
//!
//! [`ToLinear`] fixes this by giving every space exactly two methods
//! (`to_linear`/`from_linear`) that route through [`LinearRgb`] as a single
//! hub. [`ConvertSpace::convert`] then lets you go from *any* space to *any*
//! other space in one call, including combinations that have no hand-written
//! `From` impl today (e.g. `Hsl` directly to `Oklab`).
//!
//! The existing `From` impls in [`convert.rs`][super::convert] are kept as-is
//! for ergonomics and backwards compatibility (`Hsl::from(srgb)` reads better
//! than `srgb.convert::<Hsl>()` for the common conversions), but every space
//! also implements [`ToLinear`], so it participates in the general hub for
//! everything else.
//!
//! ## Examples
//!
//! ```rust
//! use gem::space::{ConvertSpace as _, Hsl, Oklab};
//!
//! // No hand-written `From<Hsl> for Oklab` impl exists, but the hub makes it
//! // a one-liner anyway.
//! let hsl = Hsl::new(0.6, 0.8, 0.5);
//! let oklab: Oklab = hsl.convert();
//! assert!(oklab.l > 0.0 && oklab.l < 1.0);
//! ```

use crate::space::{Hsl, Hsv, LinearRgb, Oklab, Srgb};

/// A color space that can round-trip through [`LinearRgb`], the hub every
/// other space converts through.
///
/// Implement this (and nothing else) to make a custom color space
/// interoperate with every space in this crate via [`ConvertSpace::convert`].
pub trait ToLinear: Copy {
    /// Converts `self` to linear light RGB.
    #[must_use]
    fn to_linear(self) -> LinearRgb;

    /// Converts from linear light RGB to `Self`.
    #[must_use]
    fn from_linear(c: LinearRgb) -> Self;
}

impl ToLinear for LinearRgb {
    fn to_linear(self) -> LinearRgb {
        self
    }

    fn from_linear(c: LinearRgb) -> Self {
        c
    }
}

impl ToLinear for Srgb {
    fn to_linear(self) -> LinearRgb {
        LinearRgb::from(self)
    }

    fn from_linear(c: LinearRgb) -> Self {
        Self::from(c)
    }
}

impl ToLinear for Hsl {
    fn to_linear(self) -> LinearRgb {
        LinearRgb::from(Srgb::from(self))
    }

    fn from_linear(c: LinearRgb) -> Self {
        Self::from(Srgb::from(c))
    }
}

impl ToLinear for Hsv {
    fn to_linear(self) -> LinearRgb {
        LinearRgb::from(Srgb::from(self))
    }

    fn from_linear(c: LinearRgb) -> Self {
        Self::from(Srgb::from(c))
    }
}

impl ToLinear for Oklab {
    fn to_linear(self) -> LinearRgb {
        LinearRgb::from(self)
    }

    fn from_linear(c: LinearRgb) -> Self {
        Self::from(c)
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl ToLinear for crate::space::Oklch {
    fn to_linear(self) -> LinearRgb {
        LinearRgb::from(Oklab::from(self))
    }

    fn from_linear(c: LinearRgb) -> Self {
        Self::from(Oklab::from_linear(c))
    }
}

/// Converts between any two [`ToLinear`] color spaces, routing through
/// [`LinearRgb`].
///
/// Blanket-implemented for every [`ToLinear`] type; see the
/// [module docs][self] for why this exists alongside the concrete `From`
/// impls in [`convert.rs`][super::convert].
pub trait ConvertSpace: ToLinear {
    /// Converts `self` to color space `T`.
    #[must_use]
    fn convert<T: ToLinear>(self) -> T {
        T::from_linear(self.to_linear())
    }
}

impl<S: ToLinear> ConvertSpace for S {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::Oklch;

    #[test]
    fn srgb_convert_is_identity_through_linear() {
        let c = Srgb::new(0.8, 0.3, 0.5);
        let back: Srgb = c.convert();
        assert!((back.r - c.r).abs() < 1e-5);
        assert!((back.g - c.g).abs() < 1e-5);
        assert!((back.b - c.b).abs() < 1e-5);
    }

    #[test]
    fn hsl_to_oklab_direct_conversion_with_no_hand_written_from_impl() {
        let hsl = Hsl::new(0.6, 0.8, 0.5);
        let oklab: Oklab = hsl.convert();
        let back: Hsl = oklab.convert();
        assert!((back.h - hsl.h).abs() < 0.01, "h: {} vs {}", back.h, hsl.h);
        assert!((back.s - hsl.s).abs() < 0.05, "s: {} vs {}", back.s, hsl.s);
        assert!((back.l - hsl.l).abs() < 0.05, "l: {} vs {}", back.l, hsl.l);
    }

    #[test]
    fn hsv_to_srgb_via_hub_matches_direct_from_impl() {
        let hsv = Hsv::new(0.25, 0.6, 0.9);
        let via_hub: Srgb = hsv.convert();
        let via_from = Srgb::from(hsv);
        assert!((via_hub.r - via_from.r).abs() < 1e-5);
        assert!((via_hub.g - via_from.g).abs() < 1e-5);
        assert!((via_hub.b - via_from.b).abs() < 1e-5);
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn oklch_roundtrip_via_hub() {
        let lch = Oklch::new(0.6, 0.15, 0.3);
        let linear = lch.to_linear();
        let back = Oklch::from_linear(linear);
        assert!((back.l - lch.l).abs() < 1e-4);
        assert!((back.c - lch.c).abs() < 1e-4);
        assert!((back.h - lch.h).abs() < 1e-4);
    }

    #[test]
    fn linear_rgb_identity() {
        let c = LinearRgb::new(0.1, 0.2, 0.3);
        assert_eq!(c.to_linear(), c);
        assert_eq!(LinearRgb::from_linear(c), c);
    }
}
