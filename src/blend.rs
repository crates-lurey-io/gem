//! Alpha compositing and blending operations.
//!
//! All operations work in [`Srgb`] + a separate straight-alpha channel, or
//! [`Premultiplied`] for representations that keep RGB pre-scaled by alpha.
//! For physically-correct results, convert to [`crate::space::LinearRgb`]
//! before blending and convert back afterward.
//!
//! ## Example
//!
//! ```rust
//! use gem::blend::alpha_over;
//! use gem::space::Srgb;
//!
//! // Composite a semi-transparent red over a blue background.
//! let src = Srgb::RED;
//! let dst = Srgb::BLUE;
//! let (result, result_alpha) = alpha_over(src, 0.5, dst, 1.0);
//! assert!(result.r > 0.0 && result.b > 0.0);
//! assert!((result_alpha - 1.0).abs() < 1e-5);
//! ```

use crate::space::Srgb;

/// Porter-Duff "source over destination" compositing.
///
/// Composites `src` (with `src_alpha`) on top of `dst` (with `dst_alpha`).
/// Returns the composited `(color, alpha)` pair.
///
/// Argument order follows the function name and the Porter-Duff / CSS Color 4
/// convention: `src` first, `dst` second (as in "source over destination").
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
/// // Fully opaque src covers dst entirely.
/// let (out, alpha) = alpha_over(Srgb::RED, 1.0, Srgb::BLUE, 1.0);
/// assert!((out.r - 1.0).abs() < 1e-5);
/// assert_eq!(alpha, 1.0);
///
/// // Fully transparent src: result equals dst.
/// let (out, alpha) = alpha_over(Srgb::RED, 0.0, Srgb::BLUE, 1.0);
/// assert!((out.b - 1.0).abs() < 1e-5);
/// ```
#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn alpha_over(src: Srgb, src_alpha: f32, dst: Srgb, dst_alpha: f32) -> (Srgb, f32) {
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

/// A color whose RGB channels have already been multiplied by its alpha.
///
/// Premultiplied alpha is the representation many GPU APIs and compositing
/// pipelines use internally, since it avoids a division per pixel during
/// blending (straight-alpha `alpha_over` still needs one).
///
/// This is a distinct type (rather than the raw `[f32; 4]` this crate used to
/// return) so "is this premultiplied or straight alpha?" is answered by the
/// type system instead of a comment.
///
/// ## Examples
///
/// ```rust
/// use gem::{blend::Premultiplied, space::Srgb};
///
/// let pre = Premultiplied::new(Srgb::RED, 0.5);
/// assert!((pre.color.r - 0.5).abs() < 1e-5);
///
/// let (color, alpha) = pre.straight();
/// assert!((color.r - 1.0).abs() < 1e-5);
/// assert!((alpha - 0.5).abs() < 1e-5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Premultiplied {
    /// Red, green, and blue channels, each already scaled by [`alpha`][Self::alpha].
    pub color: Srgb,
    /// The alpha value `color` was premultiplied by.
    pub alpha: f32,
}

impl Premultiplied {
    /// Premultiplies `color`'s channels by `alpha`.
    #[must_use]
    pub fn new(color: Srgb, alpha: f32) -> Self {
        Self {
            color: Srgb::new(color.r * alpha, color.g * alpha, color.b * alpha),
            alpha,
        }
    }

    /// Recovers the straight-alpha `(color, alpha)` pair.
    ///
    /// Returns `(Srgb::BLACK, 0.0)` if [`alpha`][Self::alpha] is effectively zero
    /// (dividing by it would be meaningless).
    #[must_use]
    pub fn straight(self) -> (Srgb, f32) {
        if self.alpha < f32::EPSILON {
            return (Srgb::BLACK, 0.0);
        }
        let inv = 1.0 / self.alpha;
        (
            Srgb::new(self.color.r * inv, self.color.g * inv, self.color.b * inv),
            self.alpha,
        )
    }
}

/// Interpolates between two `(color, alpha)` pairs by `t`.
///
/// Both the color (via [`Mix`][crate::Mix]) and the alpha are interpolated;
/// this is the straight-alpha counterpart of mixing two colors that are already
/// known to have the same opacity.
///
/// ## Examples
///
/// ```rust
/// use gem::blend::mix;
/// use gem::space::Srgb;
///
/// let (c, a) = mix(Srgb::RED, 1.0, Srgb::BLUE, 0.0, 0.5);
/// assert!((c.r - 0.5).abs() < 1e-5);
/// assert!((c.b - 0.5).abs() < 1e-5);
/// assert!((a - 0.5).abs() < 1e-5);
/// ```
#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn mix(ca: Srgb, aa: f32, cb: Srgb, ab: f32, t: f32) -> (Srgb, f32) {
    use crate::Mix as _;
    (ca.mix(cb, t), aa + (ab - aa) * t)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn alpha_over_opaque_src_covers_dst() {
        let (out, a) = alpha_over(Srgb::RED, 1.0, Srgb::BLUE, 1.0);
        assert!((out.r - 1.0).abs() < 1e-5);
        assert!(out.b.abs() < 1e-5);
        assert!((a - 1.0).abs() < 1e-5);
    }

    #[test]
    fn alpha_over_transparent_src_shows_dst() {
        let (out, a) = alpha_over(Srgb::RED, 0.0, Srgb::BLUE, 1.0);
        assert!((out.b - 1.0).abs() < 1e-5);
        assert!(out.r.abs() < 1e-5);
        assert!((a - 1.0).abs() < 1e-5);
    }

    #[test]
    fn alpha_over_half_alpha_blends() {
        let (out, out_a) = alpha_over(Srgb::RED, 0.5, Srgb::BLUE, 1.0);
        // out_alpha = 0.5 + 1.0 * 0.5 = 1.0
        assert!((out_a - 1.0).abs() < 1e-5);
        // red contribution: 0.5 * (1/1.0) = 0.5
        assert!((out.r - 0.5).abs() < 1e-5, "r={}", out.r);
        // blue contribution: 1.0 * 0.5 * (1/1.0) = 0.5
        assert!((out.b - 0.5).abs() < 1e-5, "b={}", out.b);
    }

    #[test]
    fn alpha_over_both_transparent_returns_black() {
        let (out, a) = alpha_over(Srgb::RED, 0.0, Srgb::BLUE, 0.0);
        assert_eq!(a, 0.0);
        assert_eq!(out, Srgb::BLACK);
    }

    #[test]
    fn premultiply_half() {
        let pre = Premultiplied::new(Srgb::RED, 0.5);
        assert!((pre.color.r - 0.5).abs() < 1e-5);
        assert!(pre.color.g.abs() < 1e-5);
        assert!(pre.color.b.abs() < 1e-5);
        assert!((pre.alpha - 0.5).abs() < 1e-5);
    }

    #[test]
    fn premultiply_straight_roundtrip() {
        let pre = Premultiplied::new(Srgb::new(0.8, 0.4, 0.2), 0.75);
        let (color, alpha) = pre.straight();
        assert!((color.r - 0.8).abs() < 1e-5);
        assert!((color.g - 0.4).abs() < 1e-5);
        assert!((color.b - 0.2).abs() < 1e-5);
        assert!((alpha - 0.75).abs() < 1e-5);
    }

    #[test]
    fn straight_zero_alpha() {
        let pre = Premultiplied {
            color: Srgb::new(0.5, 0.5, 0.5),
            alpha: 0.0,
        };
        let (color, alpha) = pre.straight();
        assert_eq!(alpha, 0.0);
        assert_eq!(color, Srgb::BLACK);
    }

    #[test]
    fn mix_colors() {
        let (c, a) = mix(Srgb::RED, 1.0, Srgb::BLUE, 0.0, 0.5);
        assert!((c.r - 0.5).abs() < 1e-5);
        assert!((c.b - 0.5).abs() < 1e-5);
        assert!((a - 0.5).abs() < 1e-5);
    }
}
