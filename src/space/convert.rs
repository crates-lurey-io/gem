//! Conversion implementations between color spaces and pixel formats.
//!
//! Oklab matrix multiplication is more readable as `a * b + c * d` than as
//! nested `mul_add` chains. The compiler often FMA-optimizes this anyway.
#![allow(clippy::suboptimal_flops)]

use crate::space::{
    math::{abs, rem_euclid},
    Hsl, Hsv, LinearRgb, Oklab, Srgb,
};

// ── Srgb ↔ LinearRgb ──────────────────────────────────────────────────────────

impl From<Srgb> for LinearRgb {
    fn from(c: Srgb) -> Self {
        use crate::space::math::srgb_to_linear_channel as lin;
        Self { r: lin(c.r), g: lin(c.g), b: lin(c.b) }
    }
}

impl From<LinearRgb> for Srgb {
    fn from(c: LinearRgb) -> Self {
        use crate::space::math::linear_to_srgb_channel as enc;
        Self { r: enc(c.r), g: enc(c.g), b: enc(c.b) }
    }
}

// ── Srgb ↔ Hsl ────────────────────────────────────────────────────────────────

impl From<Srgb> for Hsl {
    // max == c.r / max == c.g: these values were literally extracted from c
    // and we're checking which one was the source. Exact equality is correct.
    #[allow(clippy::float_cmp)]
    fn from(c: Srgb) -> Self {
        let max = c.r.max(c.g).max(c.b);
        let min = c.r.min(c.g).min(c.b);
        let l = f32::midpoint(max, min);
        let delta = max - min;

        let s = if delta < f32::EPSILON || l >= 1.0 || l <= 0.0 {
            0.0
        } else {
            delta / (1.0 - abs(2.0 * l - 1.0))
        };

        let h = if delta < f32::EPSILON {
            0.0
        } else if max == c.r {
            rem_euclid((c.g - c.b) / delta, 6.0) / 6.0
        } else if max == c.g {
            ((c.b - c.r) / delta + 2.0) / 6.0
        } else {
            ((c.r - c.g) / delta + 4.0) / 6.0
        };

        Self { h, s, l }
    }
}

impl From<Hsl> for Srgb {
    // h6 is in [0, 6) because c.h is in [0, 1); truncation to u32 is intentional.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::use_self)]
    fn from(c: Hsl) -> Self {
        if c.s < f32::EPSILON {
            return Srgb::new(c.l, c.l, c.l);
        }
        let chroma = (1.0 - abs(2.0 * c.l - 1.0)) * c.s;
        let h6 = c.h * 6.0;
        let x = chroma * (1.0 - abs(rem_euclid(h6, 2.0) - 1.0));
        let m = c.l - chroma / 2.0;
        let (r1, g1, b1) = match h6 as u32 {
            0 => (chroma, x, 0.0),
            1 => (x, chroma, 0.0),
            2 => (0.0, chroma, x),
            3 => (0.0, x, chroma),
            4 => (x, 0.0, chroma),
            _ => (chroma, 0.0, x),
        };
        Self::new(r1 + m, g1 + m, b1 + m)
    }
}

// ── Srgb ↔ Hsv ────────────────────────────────────────────────────────────────

impl From<Srgb> for Hsv {
    #[allow(clippy::float_cmp)]
    fn from(c: Srgb) -> Self {
        let max = c.r.max(c.g).max(c.b);
        let min = c.r.min(c.g).min(c.b);
        let delta = max - min;
        let s = if max < f32::EPSILON { 0.0 } else { delta / max };
        let h = if delta < f32::EPSILON {
            0.0
        } else if max == c.r {
            rem_euclid((c.g - c.b) / delta, 6.0) / 6.0
        } else if max == c.g {
            ((c.b - c.r) / delta + 2.0) / 6.0
        } else {
            ((c.r - c.g) / delta + 4.0) / 6.0
        };
        Self { h, s, v: max }
    }
}

impl From<Hsv> for Srgb {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::use_self)]
    fn from(c: Hsv) -> Self {
        if c.s < f32::EPSILON {
            return Srgb::new(c.v, c.v, c.v);
        }
        let h6 = c.h * 6.0;
        let chroma = c.v * c.s;
        let x = chroma * (1.0 - abs(rem_euclid(h6, 2.0) - 1.0));
        let m = c.v - chroma;
        let (r1, g1, b1) = match h6 as u32 {
            0 => (chroma, x, 0.0),
            1 => (x, chroma, 0.0),
            2 => (0.0, chroma, x),
            3 => (0.0, x, chroma),
            4 => (x, 0.0, chroma),
            _ => (chroma, 0.0, x),
        };
        Self::new(r1 + m, g1 + m, b1 + m)
    }
}

// ── LinearRgb ↔ Oklab ─────────────────────────────────────────────────────────

impl From<LinearRgb> for Oklab {
    /// Converts linear RGB to Oklab via the reference matrices.
    fn from(c: LinearRgb) -> Self {
        use crate::space::math::cbrt;
        let l = 0.412_221_5 * c.r + 0.536_332_5 * c.g + 0.051_446_0 * c.b;
        let m = 0.211_903_5 * c.r + 0.680_699_5 * c.g + 0.107_397 * c.b;
        let s = 0.088_302_5 * c.r + 0.281_718_8 * c.g + 0.629_978_7 * c.b;
        let l_ = cbrt(l);
        let m_ = cbrt(m);
        let s_ = cbrt(s);
        Self {
            l: 0.210_454_3 * l_ + 0.793_617_8 * m_ - 0.004_072_0 * s_,
            a: 1.977_998_5 * l_ - 2.428_592_2 * m_ + 0.450_593_7 * s_,
            b: 0.025_904_0 * l_ + 0.782_771_8 * m_ - 0.808_675_8 * s_,
        }
    }
}

impl From<Oklab> for LinearRgb {
    fn from(c: Oklab) -> Self {
        let l_ = c.l + 0.396_337_8 * c.a + 0.215_803_8 * c.b;
        let m_ = c.l - 0.105_561_3 * c.a - 0.063_854_2 * c.b;
        let s_ = c.l - 0.089_484_2 * c.a - 1.291_486 * c.b;
        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;
        Self {
            r: 4.076_742 * l - 3.307_712 * m + 0.230_950 * s,
            g: -1.268_438 * l + 2.609_757 * m - 0.341_319_4 * s,
            b: -0.004_196_1 * l - 0.703_418_6 * m + 1.707_615 * s,
        }
    }
}

// ── Srgb ↔ Oklab (via LinearRgb) ─────────────────────────────────────────────

#[allow(clippy::use_self)]
impl From<Srgb> for Oklab {
    fn from(c: Srgb) -> Self {
        Oklab::from(LinearRgb::from(c))
    }
}

#[allow(clippy::use_self)]
impl From<Oklab> for Srgb {
    fn from(c: Oklab) -> Self {
        Srgb::from(LinearRgb::from(c))
    }
}

// ── Oklab ↔ Oklch ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "std", feature = "libm"))]
#[allow(clippy::use_self)]
impl From<Oklab> for crate::space::Oklch {
    fn from(c: Oklab) -> Self {
        use crate::space::math::{atan2, rem_euclid, sqrt};
        use core::f32::consts::TAU;
        let chroma = sqrt(c.a * c.a + c.b * c.b);
        let h = rem_euclid(atan2(c.b, c.a) / TAU, 1.0);
        crate::space::Oklch { l: c.l, c: chroma, h }
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
impl From<crate::space::Oklch> for Oklab {
    fn from(c: crate::space::Oklch) -> Self {
        use crate::space::math::{cos, sin};
        use core::f32::consts::TAU;
        let angle = c.h * TAU;
        Self { l: c.l, a: c.c * cos(angle), b: c.c * sin(angle) }
    }
}

// ── Srgb ↔ Oklch ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "std", feature = "libm"))]
#[allow(clippy::use_self)]
impl From<Srgb> for crate::space::Oklch {
    fn from(c: Srgb) -> Self {
        crate::space::Oklch::from(Oklab::from(c))
    }
}

#[cfg(any(feature = "std", feature = "libm"))]
#[allow(clippy::use_self)]
impl From<crate::space::Oklch> for Srgb {
    fn from(c: crate::space::Oklch) -> Self {
        Srgb::from(Oklab::from(c))
    }
}

// Pixel format ↔ Srgb conversions live in `space::convert_rgb`: a single
// blanket impl over `RgbColor + RgbChannelScale` covers every pixel format
// (including custom ones), instead of a hand-written impl per format here.

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::rgb::Rgb888;

    #[test]
    fn srgb_from_rgb888_red() {
        let pixel = Rgb888::from_rgb(255, 0, 0);
        let c = Srgb::from(pixel);
        assert!((c.r - 1.0).abs() < 1e-5);
        assert!(c.g.abs() < 1e-5);
        assert!(c.b.abs() < 1e-5);
    }

    #[test]
    fn rgb888_from_srgb_roundtrip() {
        let srgb = Srgb::new(1.0, 0.502, 0.0);
        let pixel = Rgb888::from(srgb);
        let back = Srgb::from(pixel);
        assert!((back.r - srgb.r).abs() < 0.005);
        assert!((back.g - srgb.g).abs() < 0.005);
    }

    #[test]
    fn srgb_linear_rgb_roundtrip() {
        let original = Srgb::new(0.8, 0.4, 0.2);
        let linear = LinearRgb::from(original);
        let back = Srgb::from(linear);
        assert!((back.r - original.r).abs() < 0.001);
        assert!((back.g - original.g).abs() < 0.001);
        assert!((back.b - original.b).abs() < 0.001);
    }

    #[test]
    fn hsl_srgb_roundtrip() {
        let c = Srgb::new(0.8, 0.3, 0.5);
        let hsl = Hsl::from(c);
        let back = Srgb::from(hsl);
        assert!((back.r - c.r).abs() < 1e-5);
        assert!((back.g - c.g).abs() < 1e-5);
        assert!((back.b - c.b).abs() < 1e-5);
    }

    #[test]
    fn hsv_srgb_roundtrip() {
        let c = Srgb::new(0.6, 0.2, 0.9);
        let hsv = Hsv::from(c);
        let back = Srgb::from(hsv);
        assert!((back.r - c.r).abs() < 1e-5);
        assert!((back.g - c.g).abs() < 1e-5);
        assert!((back.b - c.b).abs() < 1e-5);
    }

    #[test]
    fn oklab_srgb_roundtrip() {
        let c = Srgb::new(0.8, 0.3, 0.5);
        let lab = Oklab::from(c);
        let back = Srgb::from(lab).clamp();
        assert!((back.r - c.r).abs() < 0.001);
        assert!((back.g - c.g).abs() < 0.001);
        assert!((back.b - c.b).abs() < 0.001);
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn oklch_srgb_roundtrip() {
        use crate::space::Oklch;
        let c = Srgb::new(0.8, 0.3, 0.5);
        let lch = Oklch::from(c);
        let back = Srgb::from(lch).clamp();
        assert!((back.r - c.r).abs() < 0.001);
        assert!((back.g - c.g).abs() < 0.001);
        assert!((back.b - c.b).abs() < 0.001);
    }

    #[cfg(any(feature = "std", feature = "libm"))]
    #[test]
    fn oklch_oklab_roundtrip() {
        use crate::space::Oklch;
        let lab = Oklab::new(0.6, 0.15, -0.1);
        let lch = Oklch::from(lab);
        let back = Oklab::from(lch);
        assert!((back.l - lab.l).abs() < 1e-5);
        assert!((back.a - lab.a).abs() < 1e-5);
        assert!((back.b - lab.b).abs() < 1e-5);
    }
}
