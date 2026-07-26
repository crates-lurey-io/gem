//! Linear interpolation between two colors of the same type.

use crate::channel::MixChannel;
use crate::rgb::{HasBlue, HasGreen, HasRed, RgbColor};

/// Interpolates between two colors of the same type by a factor `t`.
///
/// `t == 0.0` returns `self`; `t == 1.0` returns `other`. What "between" means
/// is up to the implementation, and deliberately differs by layer:
///
/// - **Pixel formats** ([`crate::rgb`]) interpolate per channel, in each
///   channel's native domain, with no color-space conversion. A blanket impl
///   covers every [`RgbColor`] whose channels implement [`MixChannel`], which
///   is all of them. Alpha and padding bits are copied from `self`; for
///   alpha-aware compositing see [`crate::blend`].
/// - **Color spaces** ([`crate::space`]) interpolate in their own space, which
///   for the cylindrical ones ([`Hsl`], [`Hsv`], [`Oklch`]) means taking the
///   shortest path around the hue wheel rather than the numeric average.
///
/// Interpolating gamma-encoded channels (which is what the pixel-format impls
/// do) is fast and is what most 2D blitters want. For physically-correct light
/// mixing, convert to [`LinearRgb`] first; for perceptually-even gradients,
/// convert to [`Oklab`].
///
/// For an all-integer, `const`-callable mix over `u8` channels, use
/// [`mix_u8`][crate::channel::mix_u8] — this trait's methods cannot be `const`
/// because trait methods are not const-callable on stable Rust.
///
/// [`Hsl`]: crate::space::Hsl
/// [`Hsv`]: crate::space::Hsv
/// [`Oklab`]: crate::space::Oklab
/// [`Oklch`]: crate::space::Oklch
/// [`LinearRgb`]: crate::space::LinearRgb
///
/// ## Examples
///
/// ```rust
/// use gem::Mix;
/// use gem::rgb::{Rgb888, HasRed as _, HasGreen as _, HasBlue as _};
///
/// let a = Rgb888::from_rgb(0, 0, 0);
/// let b = Rgb888::from_rgb(255, 255, 255);
///
/// // Endpoints are exact.
/// assert_eq!(a.mix(b, 0.0), a);
/// assert_eq!(a.mix(b, 1.0), b);
///
/// // Midpoint rounds half away from zero: 127.5 -> 128.
/// let mid = a.mix(b, 0.5);
/// assert_eq!((mid.red(), mid.green(), mid.blue()), (128, 128, 128));
/// ```
pub trait Mix: Sized {
    /// Interpolates `self` toward `other` by `t`.
    ///
    /// `t == 0.0` returns `self`; `t == 1.0` returns `other`.
    #[must_use]
    fn mix(self, other: Self, t: f32) -> Self;

    /// Interpolates `self` toward `other` by `t`, in place.
    fn mix_assign(&mut self, other: Self, t: f32)
    where
        Self: Copy,
    {
        *self = self.mix(other, t);
    }
}

impl<C> Mix for C
where
    C: RgbColor,
    <C as HasRed>::Component: MixChannel,
    <C as HasGreen>::Component: MixChannel,
    <C as HasBlue>::Component: MixChannel,
{
    fn mix(self, other: Self, t: f32) -> Self {
        let red = self.red().mix_channel(other.red(), t);
        let green = self.green().mix_channel(other.green(), t);
        let blue = self.blue().mix_channel(other.blue(), t);
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
    use crate::rgb::{Bgr888, Rgb, Rgb565, Rgb888, RgbF32};

    #[test]
    fn rgb888_mix() {
        let a = Rgb888::from_rgb(0, 0, 255);
        let b = Rgb888::from_rgb(255, 0, 0);
        assert_eq!(a.mix(b, 0.5), Rgb888::from_rgb(128, 0, 128));
    }

    #[test]
    fn mix_assign_matches_mix() {
        let a = Rgb888::from_rgb(0, 0, 255);
        let b = Rgb888::from_rgb(255, 0, 0);
        let mut c = a;
        c.mix_assign(b, 0.5);
        assert_eq!(c, a.mix(b, 0.5));
    }

    #[test]
    fn bgr888_mix_channels_track_color_not_position() {
        let a = Bgr888::from_bgr(0, 0, 255);
        let b = Bgr888::from_bgr(255, 0, 0);
        assert_eq!(a.mix(b, 0.5), Bgr888::from_bgr(128, 0, 128));
    }

    #[test]
    fn rgb565_mix_in_native_domain() {
        // Red channel is 5-bit (0..=31); interpolation is in that domain.
        let a = Rgb565::from_rgb(0, 0, 0);
        let b = Rgb565::from_rgb(31, 63, 31);
        let mid = a.mix(b, 0.5);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (16, 32, 16)); // 15.5->16, 31.5->32
    }

    #[test]
    fn rgbf32_mix_unclamped() {
        let a = RgbF32::from_rgb(0.0, -1.0, 0.0);
        let b = RgbF32::from_rgb(1.0, 1.0, 2.0);
        let mid = a.mix(b, 0.5);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (0.5, 0.0, 1.0));
    }

    #[test]
    fn custom_rgb_u16_mix() {
        let a: Rgb<u16> = Rgb::from_rgb(0, 0, 0);
        let b: Rgb<u16> = Rgb::from_rgb(65535, 0, 1000);
        let mid = a.mix(b, 0.5);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (32768, 0, 500)); // 32767.5->32768
    }

    #[test]
    fn rgba_preserves_alpha() {
        // Alpha (and any padding) is copied from the first operand; only RGB is
        // interpolated. Build via the wrapper constructor so the alpha field is
        // set deterministically regardless of host endianness.
        let a = AlphaFirst::<u8, Rgb888>::with_color(200, Rgb888::from_rgb(0, 0, 0));
        let b = AlphaFirst::<u8, Rgb888>::with_color(50, Rgb888::from_rgb(255, 255, 255));
        let mid = a.mix(b, 0.5);
        assert_eq!(mid.alpha(), 200);
        assert_eq!((mid.red(), mid.green(), mid.blue()), (128, 128, 128));
    }
}
