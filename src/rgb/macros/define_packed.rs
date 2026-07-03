/// Shared boilerplate for a `u16`-backed packed pixel format: the `new`
/// constructor, `From<u16>`/`Into<u16>`, and hex formatting.
macro_rules! impl_packed_u16_boilerplate {
    ($name:ident) => {
        impl $name {
            /// Creates a new color directly from its packed integer representation.
            #[must_use]
            pub const fn new(packed: u16) -> Self {
                Self { packed }
            }
        }

        impl From<u16> for $name {
            fn from(packed: u16) -> Self {
                Self::new(packed)
            }
        }

        impl From<$name> for u16 {
            fn from(color: $name) -> Self {
                color.packed
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.packed, f)
            }
        }

        impl core::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.packed, f)
            }
        }
    };
}

pub(crate) use impl_packed_u16_boilerplate;

/// Defines a `u16`-backed packed RGB pixel format (no alpha channel) from a
/// single bit-layout table.
///
/// Generates the struct, `new`/`from_rgb` constructors, `From<u16>`/
/// `Into<u16>`, hex formatting, [`HasRed`][crate::rgb::HasRed]/
/// [`HasGreen`][crate::rgb::HasGreen]/[`HasBlue`][crate::rgb::HasBlue], and
/// [`RgbChannelScale`][crate::space::RgbChannelScale] (which makes the type
/// convertible to/from [`Srgb`][crate::space::Srgb] with correct bit-depth
/// scaling — a 5-bit channel normalizes against `31`, not `255`).
///
/// ## Example
///
/// ```ignore
/// define_packed_rgb! {
///     /// A 16-bit packed RGB color.
///     pub struct Rgb565 {
///         red:   5 @ 11,
///         green: 6 @ 5,
///         blue:  5 @ 0,
///     }
/// }
/// ```
macro_rules! define_packed_rgb {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            red:   $rbits:literal @ $rshift:literal,
            green: $gbits:literal @ $gshift:literal,
            blue:  $bbits:literal @ $bshift:literal $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
        #[repr(transparent)]
        $vis struct $name {
            packed: u16,
        }

        crate::rgb::macros::impl_packed_u16_boilerplate!($name);

        impl $name {
            /// Creates a new color from individual red, green, and blue component values.
            ///
            /// This is a **lossy** conversion: values are truncated to the channel's bit width.
            #[must_use]
            pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
                let packed = ((r as u16 & ((1u16 << $rbits) - 1)) << $rshift)
                    | ((g as u16 & ((1u16 << $gbits) - 1)) << $gshift)
                    | ((b as u16 & ((1u16 << $bbits) - 1)) << $bshift);
                Self { packed }
            }
        }

        crate::rgb::macros::impl_rgb_packed!(
            $name,
            red:   { shift: $rshift, mask: (1u16 << $rbits) - 1, clear: !(((1u16 << $rbits) - 1) << $rshift) },
            green: { shift: $gshift, mask: (1u16 << $gbits) - 1, clear: !(((1u16 << $gbits) - 1) << $gshift) },
            blue:  { shift: $bshift, mask: (1u16 << $bbits) - 1, clear: !(((1u16 << $bbits) - 1) << $bshift) }
        );

        impl crate::space::RgbChannelScale for $name {
            const RED_MAX: f32 = ((1u16 << $rbits) - 1) as f32;
            const GREEN_MAX: f32 = ((1u16 << $gbits) - 1) as f32;
            const BLUE_MAX: f32 = ((1u16 << $bbits) - 1) as f32;
        }

        impl From<crate::space::Srgb> for $name {
            fn from(c: crate::space::Srgb) -> Self {
                crate::space::FromSrgb::from_srgb(c)
            }
        }
    };
}

pub(crate) use define_packed_rgb;

/// Defines a `u16`-backed packed ARGB pixel format from a single bit-layout
/// table. See [`define_packed_rgb!`] for the RGB-only equivalent; this adds
/// an alpha field and [`HasAlpha`][crate::alpha::HasAlpha].
///
/// ## Example
///
/// ```ignore
/// define_packed_argb! {
///     /// A 16-bit packed ARGB color.
///     pub struct Argb4444 {
///         alpha: 4 @ 12,
///         red:   4 @ 8,
///         green: 4 @ 4,
///         blue:  4 @ 0,
///     }
/// }
/// ```
macro_rules! define_packed_argb {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            alpha: $abits:literal @ $ashift:literal,
            red:   $rbits:literal @ $rshift:literal,
            green: $gbits:literal @ $gshift:literal,
            blue:  $bbits:literal @ $bshift:literal $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
        #[repr(transparent)]
        $vis struct $name {
            packed: u16,
        }

        crate::rgb::macros::impl_packed_u16_boilerplate!($name);

        impl $name {
            /// Creates a new color from individual alpha, red, green, and blue component values.
            ///
            /// This is a **lossy** conversion: values are truncated to each channel's bit width.
            #[must_use]
            pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
                let packed = ((a as u16 & ((1u16 << $abits) - 1)) << $ashift)
                    | ((r as u16 & ((1u16 << $rbits) - 1)) << $rshift)
                    | ((g as u16 & ((1u16 << $gbits) - 1)) << $gshift)
                    | ((b as u16 & ((1u16 << $bbits) - 1)) << $bshift);
                Self { packed }
            }

            /// Creates a new, fully-opaque color from individual red, green, and blue values.
            #[must_use]
            #[expect(
                clippy::cast_possible_truncation,
                reason = "alpha bit width is always <= 8, so the max value always fits in a u8"
            )]
            pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
                Self::from_argb(((1u16 << $abits) - 1) as u8, r, g, b)
            }
        }

        crate::rgb::macros::impl_rgb_packed!(
            $name,
            red:   { shift: $rshift, mask: (1u16 << $rbits) - 1, clear: !(((1u16 << $rbits) - 1) << $rshift) },
            green: { shift: $gshift, mask: (1u16 << $gbits) - 1, clear: !(((1u16 << $gbits) - 1) << $gshift) },
            blue:  { shift: $bshift, mask: (1u16 << $bbits) - 1, clear: !(((1u16 << $bbits) - 1) << $bshift) }
        );

        crate::rgb::macros::impl_with_alpha_packed!(
            $name,
            $ashift,
            (1u16 << $abits) - 1,
            !(((1u16 << $abits) - 1) << $ashift)
        );

        impl crate::space::RgbChannelScale for $name {
            const RED_MAX: f32 = ((1u16 << $rbits) - 1) as f32;
            const GREEN_MAX: f32 = ((1u16 << $gbits) - 1) as f32;
            const BLUE_MAX: f32 = ((1u16 << $bbits) - 1) as f32;
        }

        impl From<crate::space::Srgb> for $name {
            fn from(c: crate::space::Srgb) -> Self {
                crate::space::FromSrgb::from_srgb(c)
            }
        }
    };
}

pub(crate) use define_packed_argb;
