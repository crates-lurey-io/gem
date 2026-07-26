//! Color representations and conversions.
//!
//! `gem` is organized into two complementary layers:
//!
//! - **Pixel formats** (`rgb`, `gray`, `alpha`, `channel`): zero-cost,
//!   memory-layout-accurate types for GPU buffers, PNG encoding, and hardware
//!   interop, plus `const` integer channel arithmetic. These are
//!   `no_std`-compatible and need no math backend at all.
//!
//! - **Color spaces** (`space`, `named`, `blend`): perceptual and working-space types
//!   for manipulation, interpolation, and compositing. These are also `no_std`-compatible
//!   but need `std` or `libm`; see the [`space`] module documentation for accuracy notes.
//!
//! ## Quick start
//!
//! ```rust
//! # #[cfg(feature = "space")] {
//! use gem::prelude::*;
//! use gem::space::{Hsl, Srgb};
//! use gem::rgb::Rgb888;
//!
//! // Start with a pixel format color
//! let pixel = Rgb888::from_rgb(200, 50, 100);
//!
//! // Convert to a color space, manipulate, convert back
//! let hsl = Hsl::from(Srgb::from(pixel));
//! let lighter: Rgb888 = Srgb::from(hsl.lighten(0.15)).into();
//! assert!(lighter.red() >= pixel.red() || lighter.green() >= pixel.green());
//! # }
//! ```
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `std` | Turns on [`space`], using the standard library's math |
//! | `libm` | Turns on [`space`] in `no_std` environments, using `libm`'s math |
//! | `libm-arch` | Architecture-specific `libm` intrinsics; only meaningful with `libm` |
//! | `blend` | Enables the [`blend`] module (Porter-Duff compositing, premultiplied alpha); implies `space` |
//! | `bytemuck` | Derives `Pod`/`Zeroable` for all pixel format types, enabling zero-copy buffer casting |
//! | `serde` | Derives `Serialize`/`Deserialize` for pixel format and color space types |
//! | `space` | Turned on by the four above; you should not need to name it directly |
//!
//! No feature is enabled by default, and the default build is the full
//! pixel-format layer: [`rgb`], [`gray`], [`alpha`], [`channel`], and [`Mix`]
//! all work with `default-features = false` and no math backend.
//!
//! [`space`], [`named`], and [`blend`] need accurate `x^2.4` gamma correction
//! and trigonometry, so they are compiled only when `std` or `libm` is enabled.
//! Enable whichever fits your target; a bare `cargo add gem` gets you the
//! pixel-format layer alone.
//!
//! ## Rounding
//!
//! Every integer channel result in this crate is **round-to-nearest, ties away
//! from zero**. This holds for [`channel`]'s `const` integer math, for
//! [`Mix`] on integer-channel formats, and for every conversion out of
//! [`space`] into a pixel format.
//!
//! This is a deliberate crate-wide guarantee rather than an implementation
//! detail. Compositing pipelines routinely mix libraries that each pick their
//! own `/255` convention, and the cheap truncating one (`(v + (v >> 8) + 1) >> 8`,
//! exactly `floor(v / 255)`) biases every channel downward by up to a full
//! step, so an image that is composited repeatedly visibly darkens. No
//! operation here does that, and the tests in [`channel`] prove it
//! exhaustively rather than by sampling.
//!
//! ## Non-goals
//!
//! `gem` intentionally does not (and has no plans to) provide:
//!
//! - **CMYK, CIE XYZ/Lab, or wide-gamut spaces** (Display P3, Rec. 2020). The
//!   `space` module covers the common sRGB-adjacent working spaces; anything
//!   requiring full color management is out of scope.
//! - **A heterogeneous `Color` sum type or enum.** Every pixel format and
//!   color space here is a distinct, monomorphic type with no common
//!   object-safe supertype. If you need "a `Vec` of colors that might be
//!   `Hsl` or might be `Oklab`," define your own enum over the concrete types
//!   you use — this mirrors `bevy_color`'s deliberate choice to avoid the
//!   API-surface and monomorphization cost of a fully generic color type.
//! - **String/CSS parsing beyond hex** (`Srgb::from_hex`). Named colors are
//!   provided as constants ([`named`]), not a parser for arbitrary CSS color
//!   syntax (`rgb()`, `hsl()`, `oklch()` functional notation, etc.).

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "std", test))]
extern crate std;

pub mod alpha;
pub mod channel;
pub mod gray;
pub mod prelude;
pub mod rgb;

mod mix;
pub use mix::Mix;

#[cfg(feature = "blend")]
#[cfg_attr(docsrs, doc(cfg(feature = "blend")))]
pub mod blend;

#[cfg(feature = "space")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "libm"))))]
pub mod named;

#[cfg(feature = "space")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "libm"))))]
pub mod space;
