//! Per-channel linear interpolation across pixel formats.
//!
//! Unlike [`crate::space::Srgb::lerp`], which works in normalized `[0.0, 1.0]`
//! floats, these operate directly in each format's native channel domain
//! (`u8`, `u16`, `f32`, or a sub-byte packed field). No color-space conversion
//! is performed: the interpolation is per-channel in whatever encoding the
//! pixel already uses (typically gamma-encoded sRGB). Integer channels round to
//! nearest with ties away from zero.
//!
//! These require neither `std` nor `libm`: integer rounding is done with
//! `floor(x + 0.5)`, so blending is available in the strictest `no_std` builds.

use crate::rgb::{HasBlue, HasGreen, HasRed, RgbColor};

/// Linearly interpolates a single channel value toward `other` by `t`.
///
/// `t = 0.0` returns `self`; `t = 1.0` returns `other`. Integer channels round
/// to nearest, ties away from zero, and clamp to the channel's representable
/// range; floating-point channels interpolate exactly and are not clamped.
pub trait LerpChannel: Copy {
    /// Interpolates `self` toward `other` by `t`.
    #[must_use]
    fn lerp_channel(self, other: Self, t: f32) -> Self;
}

impl LerpChannel for u8 {
    #[inline]
    #[allow(clippy::suboptimal_flops)]
    fn lerp_channel(self, other: Self, t: f32) -> Self {
        // Interpolate in the 0..=255 domain and round once, half away from
        // zero. Staying in this domain (rather than normalizing to [0, 1] and
        // back) makes the result an exact round-to-nearest of a + (b - a) * t,
        // so t = 0.5 yields the true midpoint (e.g. 0, 255 -> 128).
        let a = f32::from(self);
        let b = f32::from(other);
        let v = (a + (b - a) * t).clamp(0.0, 255.0);
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "v is clamped to [0, 255]; +0.5 then truncation rounds half away from zero"
        )]
        {
            (v + 0.5) as Self
        }
    }
}

impl LerpChannel for u16 {
    #[inline]
    #[allow(clippy::suboptimal_flops)]
    fn lerp_channel(self, other: Self, t: f32) -> Self {
        let a = f32::from(self);
        let b = f32::from(other);
        let v = (a + (b - a) * t).clamp(0.0, 65535.0);
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "v is clamped to [0, 65535]; +0.5 then truncation rounds half away from zero"
        )]
        {
            (v + 0.5) as Self
        }
    }
}

impl LerpChannel for f32 {
    #[inline]
    #[allow(clippy::suboptimal_flops)]
    fn lerp_channel(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

/// Per-channel linear interpolation between two colors of the same format.
///
/// Implemented for every [`RgbColor`] whose channels implement [`LerpChannel`]
/// (all built-in RGB formats: [`Rgb888`], [`Rgb565`], [`Bgr888`], [`Rgbf32`],
/// the ARGB/ABGR types, and custom [`Rgb<T>`]/[`Bgr<T>`]).
///
/// Only the red, green, and blue channels are interpolated. Any other bits
/// (alpha or padding) are copied unchanged from `self`. For alpha-aware
/// compositing that interpolates the alpha channel, see [`crate::blend`].
///
/// [`Rgb888`]: crate::rgb::Rgb888
/// [`Rgb565`]: crate::rgb::Rgb565
/// [`Bgr888`]: crate::rgb::Bgr888
/// [`Rgbf32`]: crate::rgb::Rgbf32
/// [`Rgb<T>`]: crate::rgb::Rgb
/// [`Bgr<T>`]: crate::rgb::Bgr
///
/// ## Examples
///
/// ```rust
/// use gem::rgb::{Lerp, Rgb888, HasRed as _, HasGreen as _, HasBlue as _};
///
/// let a = Rgb888::from_rgb(0, 0, 0);
/// let b = Rgb888::from_rgb(255, 255, 255);
///
/// // Endpoints are exact.
/// assert_eq!(a.lerp(b, 0.0), a);
/// assert_eq!(a.lerp(b, 1.0), b);
///
/// // Midpoint rounds half away from zero: 127.5 -> 128.
/// let mid = a.lerp(b, 0.5);
/// assert_eq!((mid.red(), mid.green(), mid.blue()), (128, 128, 128));
/// ```
pub trait Lerp: Sized {
    /// Interpolates each color channel toward `other` by `t`.
    ///
    /// `t = 0.0` returns `self`; `t = 1.0` returns `other`.
    #[must_use]
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl<C> Lerp for C
where
    C: RgbColor,
    <C as HasRed>::Component: LerpChannel,
    <C as HasGreen>::Component: LerpChannel,
    <C as HasBlue>::Component: LerpChannel,
{
    fn lerp(self, other: Self, t: f32) -> Self {
        let red = self.red().lerp_channel(other.red(), t);
        let green = self.green().lerp_channel(other.green(), t);
        let blue = self.blue().lerp_channel(other.blue(), t);
        // Start from `self` so alpha/padding bits survive, then overwrite RGB.
        let mut out = self;
        out.set_red(red);
        out.set_green(green);
        out.set_blue(blue);
        out
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::alpha::AlphaFirst;
    use crate::rgb::{Bgr888, Rgb, Rgb565, Rgb888, Rgbf32};

    #[test]
    fn u8_endpoints_and_midpoint() {
        assert_eq!(0u8.lerp_channel(255, 0.0), 0);
        assert_eq!(0u8.lerp_channel(255, 1.0), 255);
        assert_eq!(0u8.lerp_channel(255, 0.5), 128); // 127.5 -> 128
        assert_eq!(100u8.lerp_channel(101, 0.5), 101); // 100.5 -> 101
    }

    #[test]
    fn u8_midpoint_is_exact_for_all_pairs() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                let got = a.lerp_channel(b, 0.5);
                // Round-half-away of the rational midpoint (a + b) / 2.
                let exact = u8::try_from((u16::from(a) + u16::from(b)).div_ceil(2)).unwrap();
                assert_eq!(got, exact, "a={a} b={b}");
            }
        }
    }

    #[test]
    fn u8_clamps_out_of_range_t() {
        assert_eq!(10u8.lerp_channel(20, -1.0), 0);
        assert_eq!(250u8.lerp_channel(255, 2.0), 255);
    }

    #[test]
    fn f32_channel_is_exact() {
        assert_eq!(0.0f32.lerp_channel(1.0, 0.25), 0.25);
        assert_eq!((-1.0f32).lerp_channel(1.0, 0.5), 0.0);
    }

    #[test]
    fn rgb888_lerp() {
        let a = Rgb888::from_rgb(0, 0, 255);
        let b = Rgb888::from_rgb(255, 0, 0);
        assert_eq!(a.lerp(b, 0.5), Rgb888::from_rgb(128, 0, 128));
    }

    #[test]
    fn bgr888_lerp_channels_track_color_not_position() {
        let a = Bgr888::from_bgr(0, 0, 255);
        let b = Bgr888::from_bgr(255, 0, 0);
        assert_eq!(a.lerp(b, 0.5), Bgr888::from_bgr(128, 0, 128));
    }

    #[test]
    fn rgb565_lerp_in_native_domain() {
        // Red channel is 5-bit (0..=31); interpolation is in that domain.
        let a = Rgb565::from_rgb(0, 0, 0);
        let b = Rgb565::from_rgb(31, 63, 31);
        let mid = a.lerp(b, 0.5);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (16, 32, 16)); // 15.5->16, 31.5->32
    }

    #[test]
    fn rgbf32_lerp_unclamped() {
        let a = Rgbf32::from_rgb(0.0, -1.0, 0.0);
        let b = Rgbf32::from_rgb(1.0, 1.0, 2.0);
        let mid = a.lerp(b, 0.5);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (0.5, 0.0, 1.0));
    }

    #[test]
    fn custom_rgb_u16_lerp() {
        let a: Rgb<u16> = Rgb::from_rgb(0, 0, 0);
        let b: Rgb<u16> = Rgb::from_rgb(65535, 0, 1000);
        let mid = a.lerp(b, 0.5);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (32768, 0, 500)); // 32767.5->32768
    }

    #[test]
    fn rgba_preserves_alpha() {
        // Alpha (and any padding) is copied from the first operand; only RGB is
        // interpolated. Build via the wrapper constructor so the alpha field is
        // set deterministically regardless of host endianness.
        let a = AlphaFirst::<u8, Rgb888>::with_color(200, Rgb888::from_rgb(0, 0, 0));
        let b = AlphaFirst::<u8, Rgb888>::with_color(50, Rgb888::from_rgb(255, 255, 255));
        let mid = a.lerp(b, 0.5);
        assert_eq!(mid.alpha(), 200);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (128, 128, 128));
    }
}
