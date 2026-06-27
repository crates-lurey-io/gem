//! Alpha compositing and blending operations.
//!
//! All operations work in [`Srgb`] + separate alpha channel.
//! For physically-correct results, convert to [`crate::space::LinearRgb`]
//! before blending and convert back afterward.
//!
//! ## Example
//!
//! ```rust
//! use gem::blend::{alpha_over, premultiply};
//! use gem::space::Srgb;
//!
//! // Composite a semi-transparent red over a blue background
//! let dst = Srgb::BLUE;
//! let src = Srgb::RED;
//! let (result, result_alpha) = alpha_over(dst, 1.0, src, 0.5);
//! assert!(result.r > 0.0 && result.b > 0.0);
//! assert!((result_alpha - 1.0).abs() < 1e-5);
//! ```

use crate::space::Srgb;

/// Porter-Duff "source over destination" compositing.
///
/// Composites `src` (with `src_alpha`) on top of `dst` (with `dst_alpha`).
/// Returns the composited `(color, alpha)` pair.
///
/// This implements the standard alpha-over formula:
/// ```text
/// out_alpha = src_alpha + dst_alpha * (1 - src_alpha)
/// out_color = (src * src_alpha + dst * dst_alpha * (1 - src_alpha)) / out_alpha
/// ```
///
/// ## Examples
///
/// ```rust
/// use gem::blend::alpha_over;
/// use gem::space::Srgb;
///
/// // Fully opaque src covers dst entirely
/// let (out, alpha) = alpha_over(Srgb::BLUE, 1.0, Srgb::RED, 1.0);
/// assert!((out.r - 1.0).abs() < 1e-5);
/// assert_eq!(alpha, 1.0);
///
/// // Fully transparent src: result equals dst
/// let (out, alpha) = alpha_over(Srgb::BLUE, 1.0, Srgb::RED, 0.0);
/// assert!((out.b - 1.0).abs() < 1e-5);
/// ```
#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn alpha_over(dst: Srgb, dst_alpha: f32, src: Srgb, src_alpha: f32) -> (Srgb, f32) {
    let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);
    if out_alpha < f32::EPSILON {
        return (Srgb::BLACK, 0.0);
    }
    let inv = 1.0 / out_alpha;
    let w_src = src_alpha * inv;
    let w_dst = dst_alpha * (1.0 - src_alpha) * inv;
    let out = Srgb {
        r: src.r * w_src + dst.r * w_dst,
        g: src.g * w_src + dst.g * w_dst,
        b: src.b * w_src + dst.b * w_dst,
    };
    (out, out_alpha)
}

/// Multiplies a color's channels by its alpha value (premultiplied alpha).
///
/// Premultiplied alpha is the representation used by many GPU APIs and compositing
/// operations because it avoids a division per-pixel during blending.
///
/// Returns `[r*a, g*a, b*a, a]`.
///
/// ## Examples
///
/// ```rust
/// use gem::blend::premultiply;
/// use gem::space::Srgb;
///
/// let [r, g, b, a] = premultiply(Srgb::RED, 0.5);
/// assert!((r - 0.5).abs() < 1e-5);
/// assert!((a - 0.5).abs() < 1e-5);
/// ```
#[must_use]
pub fn premultiply(color: Srgb, alpha: f32) -> [f32; 4] {
    [color.r * alpha, color.g * alpha, color.b * alpha, alpha]
}

/// Recovers a straight-alpha `(color, alpha)` pair from a premultiplied `[r, g, b, a]`.
///
/// Returns `(Srgb::BLACK, 0.0)` if alpha is effectively zero.
///
/// ## Examples
///
/// ```rust
/// use gem::blend::{premultiply, unpremultiply};
/// use gem::space::Srgb;
///
/// let pre = premultiply(Srgb::RED, 0.5);
/// let (color, alpha) = unpremultiply(pre);
/// assert!((color.r - 1.0).abs() < 1e-5);
/// assert!((alpha - 0.5).abs() < 1e-5);
/// ```
#[must_use]
pub fn unpremultiply([r, g, b, a]: [f32; 4]) -> (Srgb, f32) {
    if a < f32::EPSILON {
        return (Srgb::BLACK, 0.0);
    }
    let inv = 1.0 / a;
    (Srgb::new(r * inv, g * inv, b * inv), a)
}

/// Linearly interpolates between two `(color, alpha)` pairs by `t`.
///
/// ## Examples
///
/// ```rust
/// use gem::blend::lerp;
/// use gem::space::Srgb;
///
/// let (c, a) = lerp(Srgb::RED, 1.0, Srgb::BLUE, 0.0, 0.5);
/// assert!((c.r - 0.5).abs() < 1e-5);
/// assert!((c.b - 0.5).abs() < 1e-5);
/// assert!((a - 0.5).abs() < 1e-5);
/// ```
#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn lerp(ca: Srgb, aa: f32, cb: Srgb, ab: f32, t: f32) -> (Srgb, f32) {
    (ca.lerp(cb, t), aa + (ab - aa) * t)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn alpha_over_opaque_src_covers_dst() {
        let (out, a) = alpha_over(Srgb::BLUE, 1.0, Srgb::RED, 1.0);
        assert!((out.r - 1.0).abs() < 1e-5);
        assert!(out.b.abs() < 1e-5);
        assert!((a - 1.0).abs() < 1e-5);
    }

    #[test]
    fn alpha_over_transparent_src_shows_dst() {
        let (out, a) = alpha_over(Srgb::BLUE, 1.0, Srgb::RED, 0.0);
        assert!((out.b - 1.0).abs() < 1e-5);
        assert!(out.r.abs() < 1e-5);
        assert!((a - 1.0).abs() < 1e-5);
    }

    #[test]
    fn alpha_over_half_alpha_blends() {
        let (out, out_a) = alpha_over(Srgb::BLUE, 1.0, Srgb::RED, 0.5);
        // out_alpha = 0.5 + 1.0 * 0.5 = 1.0
        assert!((out_a - 1.0).abs() < 1e-5);
        // red contribution: 0.5 * (1/1.0) = 0.5
        assert!((out.r - 0.5).abs() < 1e-5, "r={}", out.r);
        // blue contribution: 1.0 * 0.5 * (1/1.0) = 0.5
        assert!((out.b - 0.5).abs() < 1e-5, "b={}", out.b);
    }

    #[test]
    fn alpha_over_both_transparent_returns_black() {
        let (out, a) = alpha_over(Srgb::BLUE, 0.0, Srgb::RED, 0.0);
        assert_eq!(a, 0.0);
        assert_eq!(out, Srgb::BLACK);
    }

    #[test]
    fn premultiply_half() {
        let [r, g, b, a] = premultiply(Srgb::RED, 0.5);
        assert!((r - 0.5).abs() < 1e-5);
        assert!(g.abs() < 1e-5);
        assert!(b.abs() < 1e-5);
        assert!((a - 0.5).abs() < 1e-5);
    }

    #[test]
    fn premultiply_unpremultiply_roundtrip() {
        let pre = premultiply(Srgb::new(0.8, 0.4, 0.2), 0.75);
        let (color, alpha) = unpremultiply(pre);
        assert!((color.r - 0.8).abs() < 1e-5);
        assert!((color.g - 0.4).abs() < 1e-5);
        assert!((color.b - 0.2).abs() < 1e-5);
        assert!((alpha - 0.75).abs() < 1e-5);
    }

    #[test]
    fn unpremultiply_zero_alpha() {
        let (color, alpha) = unpremultiply([0.5, 0.5, 0.5, 0.0]);
        assert_eq!(alpha, 0.0);
        assert_eq!(color, Srgb::BLACK);
    }

    #[test]
    fn lerp_colors() {
        let (c, a) = lerp(Srgb::RED, 1.0, Srgb::BLUE, 0.0, 0.5);
        assert!((c.r - 0.5).abs() < 1e-5);
        assert!((c.b - 0.5).abs() < 1e-5);
        assert!((a - 0.5).abs() < 1e-5);
    }
}
