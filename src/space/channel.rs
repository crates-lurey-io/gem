//! Channel-level scaling glue for converting pixel-format components to and
//! from the normalized `[0.0, 1.0]` floats used by [`crate::space`] color types.

use crate::space::math::{clamp, round};

/// A raw pixel channel component that can be converted to/from a plain `f32`,
/// with no range scaling applied.
///
/// This is the "storage" half of channel normalization; the "range" half (how
/// many bits a particular channel occupies) is supplied per color type by
/// [`RgbChannelScale`], since the same `u8` component type is reused both for
/// full 8-bit channels (e.g. [`Rgb888`][crate::rgb::Rgb888]) and packed
/// sub-byte channels (e.g. [`Rgb565`][crate::rgb::Rgb565], whose red channel
/// is 5 bits but is still stored in a `u8`).
pub trait Channel: Copy + Default {
    /// Converts this value to `f32`, with no scaling applied.
    fn to_f32(self) -> f32;

    /// Converts an `f32` to this channel type, rounding to nearest (ties away
    /// from zero) and clamping to the type's representable range.
    fn from_f32(v: f32) -> Self;
}

impl Channel for u8 {
    fn to_f32(self) -> f32 {
        f32::from(self)
    }

    fn from_f32(v: f32) -> Self {
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "v is clamped to [0, 255] before the cast"
        )]
        {
            round(clamp(v, 0.0, 255.0)) as Self
        }
    }
}

impl Channel for u16 {
    fn to_f32(self) -> f32 {
        f32::from(self)
    }

    fn from_f32(v: f32) -> Self {
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "v is clamped to [0, 65535] before the cast"
        )]
        {
            round(clamp(v, 0.0, 65_535.0)) as Self
        }
    }
}

impl Channel for f32 {
    fn to_f32(self) -> f32 {
        self
    }

    fn from_f32(v: f32) -> Self {
        v
    }
}

/// The maximum representable value of a native (non-packed) channel type.
///
/// Used as the [`RgbChannelScale`] source for generic pixel formats such as
/// [`Rgb<T>`][crate::rgb::Rgb], whose channel range is simply "whatever `T`
/// can hold" rather than a packed sub-range.
pub trait NativeMax: Channel {
    /// The maximum representable value, as `f32` (e.g. `255.0` for `u8`).
    const MAX: f32;
}

impl NativeMax for u8 {
    const MAX: f32 = 255.0;
}

impl NativeMax for u16 {
    const MAX: f32 = 65_535.0;
}

impl NativeMax for f32 {
    const MAX: f32 = 1.0;
}

/// Supplies the maximum representable value of each RGB channel for a
/// pixel-format type.
///
/// This is what makes generic conversion to/from [`Srgb`][crate::space::Srgb]
/// possible for *any* [`RgbColor`][crate::rgb::RgbColor] implementation,
/// packed sub-byte formats included: a type whose red channel is a 5-bit
/// field stored in a `u8` (e.g. [`Rgb565`][crate::rgb::Rgb565]) reports
/// `RED_MAX = 31.0`, while a type whose red channel is a full `u8`
/// (e.g. [`Rgb888`][crate::rgb::Rgb888]) reports `RED_MAX = 255.0`.
///
/// Implemented automatically for every built-in pixel format. Implement this
/// manually for custom formats to opt into the blanket `Srgb` conversions in
/// [`crate::space`].
pub trait RgbChannelScale: crate::rgb::RgbColor {
    /// Maximum representable value of the red channel.
    const RED_MAX: f32;
    /// Maximum representable value of the green channel.
    const GREEN_MAX: f32;
    /// Maximum representable value of the blue channel.
    const BLUE_MAX: f32;
}

/// Converts a pixel-format color from [`Srgb`][crate::space::Srgb], scaled
/// correctly for the type's channel bit depth.
///
/// Every built-in pixel format also implements `From<Srgb>` directly (so
/// `.into()` and `Type::from(srgb)` work as expected); this trait exists
/// because Rust's coherence rules don't allow a single blanket
/// `impl<C: RgbChannelScale> From<Srgb> for C` covering *every* possible
/// implementor, including ones defined outside this crate. If you implement
/// [`RgbColor`][crate::rgb::RgbColor] and [`RgbChannelScale`] for your own
/// type, implement `From<Srgb>` for it as a one-liner that calls
/// [`from_srgb`][Self::from_srgb]:
///
/// ```ignore
/// impl From<Srgb> for MyPixelFormat {
///     fn from(c: Srgb) -> Self {
///         FromSrgb::from_srgb(c)
///     }
/// }
/// ```
pub trait FromSrgb: RgbChannelScale + Default {
    /// Converts from `Srgb`, scaling each channel by this type's
    /// [`RgbChannelScale`] maximums.
    ///
    /// `Srgb` carries no alpha channel, so for alpha-carrying pixel formats
    /// (e.g. [`Argb8888`][crate::rgb::Argb8888]) the result's alpha is left
    /// at `Self::default()`'s value (typically `0`, fully transparent) —
    /// only red, green, and blue are populated from `c`.
    #[must_use]
    fn from_srgb(c: crate::space::Srgb) -> Self
    where
        <Self as crate::rgb::HasRed>::Component: Channel,
        <Self as crate::rgb::HasGreen>::Component: Channel,
        <Self as crate::rgb::HasBlue>::Component: Channel,
    {
        crate::rgb::RgbColor::from_rgb(
            Channel::from_f32(c.r.clamp(0.0, 1.0) * Self::RED_MAX),
            Channel::from_f32(c.g.clamp(0.0, 1.0) * Self::GREEN_MAX),
            Channel::from_f32(c.b.clamp(0.0, 1.0) * Self::BLUE_MAX),
        )
    }
}

impl<C: RgbChannelScale + Default> FromSrgb for C {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_roundtrip() {
        assert_eq!(u8::from_f32(255.0.to_f32()), 255);
        assert_eq!(u8::from_f32(0.0), 0);
        assert_eq!(u8::from_f32(-10.0), 0);
        assert_eq!(u8::from_f32(300.0), 255);
    }

    #[test]
    fn u16_roundtrip() {
        assert_eq!(u16::from_f32(65_535.0), 65_535);
        assert_eq!(u16::from_f32(-1.0), 0);
    }

    #[test]
    fn f32_identity() {
        assert_eq!(f32::from_f32(0.25), 0.25);
        assert_eq!(0.75f32.to_f32(), 0.75);
    }

    #[test]
    fn native_max_values() {
        assert_eq!(<u8 as NativeMax>::MAX, 255.0);
        assert_eq!(<u16 as NativeMax>::MAX, 65_535.0);
        assert_eq!(<f32 as NativeMax>::MAX, 1.0);
    }
}
