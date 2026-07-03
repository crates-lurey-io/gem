//! Internal float math helpers that work in `no_std` + `libm` or `std` contexts.
//!
//! All float operations that are not available in `core` unconditionally are
//! routed through this module.
//! Each function dispatches to either `std` or `libm` depending on which feature is active.

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!(
    "Either the 'std' or 'libm' feature must be enabled for gem's color space module. \
     Add `features = [\"std\"]` (if you have std available) or `features = [\"libm\"]` \
     (for no_std environments) to your dependency."
);

// ── float wrappers ────────────────────────────────────────────────────────────

/// Square root.
#[inline]
pub(super) fn sqrt(x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::sqrt(x);
    #[cfg(not(feature = "std"))]
    return libm::sqrtf(x);
}

/// Round to nearest integer, ties away from zero.
#[inline]
pub(super) fn round(x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::round(x);
    #[cfg(not(feature = "std"))]
    return libm::roundf(x);
}

/// Raise `x` to the power `2.4` (sRGB linearization gamma).
#[inline]
pub(super) fn pow24(x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::powf(x, 2.4_f32);
    #[cfg(not(feature = "std"))]
    return libm::powf(x, 2.4_f32);
}

/// Raise `x` to the power `1/2.4` (sRGB encoding gamma).
#[inline]
pub(super) fn pow_inv24(x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::powf(x, 1.0_f32 / 2.4_f32);
    #[cfg(not(feature = "std"))]
    return libm::powf(x, 1.0_f32 / 2.4_f32);
}

/// Four-quadrant arctangent of `y/x` in radians.
///
/// Only available when `std` or `libm` is enabled.
#[cfg(any(feature = "std", feature = "libm"))]
#[inline]
pub(super) fn atan2(y: f32, x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::atan2(y, x);
    #[cfg(not(feature = "std"))]
    return libm::atan2f(y, x);
}

/// Sine of `x` (radians).
///
/// Only available when `std` or `libm` is enabled.
#[cfg(any(feature = "std", feature = "libm"))]
#[inline]
pub(super) fn sin(x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::sin(x);
    #[cfg(not(feature = "std"))]
    return libm::sinf(x);
}

/// Cosine of `x` (radians).
///
/// Only available when `std` or `libm` is enabled.
#[cfg(any(feature = "std", feature = "libm"))]
#[inline]
pub(super) fn cos(x: f32) -> f32 {
    #[cfg(feature = "std")]
    return f32::cos(x);
    #[cfg(not(feature = "std"))]
    return libm::cosf(x);
}

// ── pure-arithmetic helpers (no libm/std needed) ─────────────────────────────

/// Clamp `x` to `[min, max]`.
#[inline]
pub(super) const fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Absolute value.
#[inline]
pub(super) const fn abs(x: f32) -> f32 {
    f32::abs(x)
}

/// Signum: +1.0, -1.0, or 0.0.
#[inline]
pub(super) fn signum(x: f32) -> f32 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Euclidean remainder — always non-negative when `rhs > 0`.
///
/// Equivalent to `x.rem_euclid(rhs)` in std.
#[inline]
pub(super) fn rem_euclid(x: f32, rhs: f32) -> f32 {
    let r = x % rhs;
    if r < 0.0 { r + abs(rhs) } else { r }
}

// ── color-specific helpers ────────────────────────────────────────────────────

/// Converts a single sRGB channel `[0, 1]` to linear light `[0, 1]`
/// using the exact IEC 61966-2-1 formula.
#[inline]
pub(super) fn srgb_to_linear_channel(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        pow24((c + 0.055) / 1.055)
    }
}

/// Converts a single linear light value `[0, 1]` to sRGB `[0, 1]`
/// using the exact IEC 61966-2-1 formula.
#[inline]
#[allow(clippy::suboptimal_flops)]
pub(super) fn linear_to_srgb_channel(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * pow_inv24(c) - 0.055
    }
}

/// Converts a channel value to a `u8` by clamping to `[0, 1]`, scaling by 255, and rounding.
///
/// The cast is safe because clamp + round guarantees a result in `[0.0, 255.0]`.
#[inline]
#[allow(clippy::suboptimal_flops)]
pub(super) fn channel_to_u8(x: f32) -> u8 {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "value is clamped to [0, 1] then scaled to [0, 255]; cast is always safe"
    )]
    {
        round(clamp(x, 0.0, 1.0) * 255.0) as u8
    }
}

/// Cube root using a bit-manipulation initial estimate and four Newton iterations.
/// Accurate to within a few ULP for all finite `f32` values.
#[inline]
#[allow(clippy::suboptimal_flops)]
pub(super) fn cbrt(x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }
    let sign = signum(x);
    let x = abs(x);
    // Bit-manipulation initial estimate
    let mut y = f32::from_bits(x.to_bits() / 3 + 0x2A51_1CD3);
    // Newton: y = (2y + x/y²) / 3
    y = (2.0 * y + x / (y * y)) / 3.0;
    y = (2.0 * y + x / (y * y)) / 3.0;
    y = (2.0 * y + x / (y * y)) / 3.0;
    y = (2.0 * y + x / (y * y)) / 3.0;
    sign * y
}

/// Linear interpolation between `a` and `b` by `t`.
#[inline]
#[allow(clippy::suboptimal_flops)]
pub(super) fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linearly interpolates a hue value in `[0, 1)`, always taking the shortest
/// path around the circle.
#[inline]
pub(super) fn lerp_hue(a: f32, b: f32, t: f32) -> f32 {
    let diff = b - a;
    let diff = if diff > 0.5 {
        diff - 1.0
    } else if diff < -0.5 {
        diff + 1.0
    } else {
        diff
    };
    #[allow(clippy::suboptimal_flops)]
    rem_euclid(a + t * diff, 1.0)
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::float_cmp_const)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_four() {
        assert!((sqrt(4.0) - 2.0).abs() < 1e-5);
    }

    #[test]
    fn round_half() {
        assert_eq!(round(1.5), 2.0);
        assert_eq!(round(1.4), 1.0);
    }

    #[test]
    fn clamp_range() {
        assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
    }

    #[test]
    fn abs_values() {
        assert_eq!(abs(-3.0), 3.0);
        assert_eq!(abs(3.0), 3.0);
        assert_eq!(abs(0.0), 0.0);
    }

    #[test]
    fn signum_values() {
        assert_eq!(signum(-5.0), -1.0);
        assert_eq!(signum(5.0), 1.0);
        assert_eq!(signum(0.0), 0.0);
    }

    #[test]
    fn rem_euclid_positive() {
        assert!((rem_euclid(7.0, 3.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rem_euclid_negative() {
        // -1 mod 6 = 5 (positive result)
        assert!((rem_euclid(-1.0, 6.0) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn cbrt_positive() {
        assert!((cbrt(27.0) - 3.0).abs() < 1e-4);
    }

    #[test]
    fn cbrt_negative() {
        assert!((cbrt(-8.0) - (-2.0)).abs() < 1e-4);
    }

    #[test]
    fn cbrt_zero() {
        assert_eq!(cbrt(0.0), 0.0);
    }

    #[test]
    fn srgb_linear_channel_zero_one() {
        assert!((srgb_to_linear_channel(0.0)).abs() < 1e-6);
        assert!((srgb_to_linear_channel(1.0) - 1.0).abs() < 1e-4);
    }

    #[test]
    fn linear_srgb_channel_zero_one() {
        assert!((linear_to_srgb_channel(0.0)).abs() < 1e-6);
        assert!((linear_to_srgb_channel(1.0) - 1.0).abs() < 1e-4);
    }

    #[test]
    fn gamma_roundtrip() {
        for &v in &[0.0_f32, 0.1, 0.5, 0.9, 1.0] {
            let lin = srgb_to_linear_channel(v);
            let back = linear_to_srgb_channel(lin);
            assert!((back - v).abs() < 1e-4, "v={v}, back={back}");
        }
    }

    #[test]
    fn lerp_hue_no_wrap() {
        assert!((lerp_hue(0.1, 0.3, 0.5) - 0.2).abs() < 1e-6);
    }

    #[test]
    fn lerp_hue_wraps_through_zero() {
        // 0.9 -> 0.1 shortest path goes through 0.0
        let h = lerp_hue(0.9, 0.1, 0.5);
        assert!(!(0.05..=0.95).contains(&h), "h={h}");
    }
}
