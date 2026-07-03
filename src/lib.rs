//! Color representations and conversions.
//!
//! `gem` is organized into two complementary layers:
//!
//! - **Pixel formats** (`rgb`, `gray`, `alpha`): zero-cost, memory-layout-accurate types
//!   for GPU buffers, PNG encoding, and hardware interop. These are `no_std`-compatible
//!   and work without any math at all.
//!
//! - **Color spaces** (`space`, `named`, `blend`): perceptual and working-space types
//!   for manipulation, interpolation, and compositing. These are also `no_std`-compatible
//!   but see the [`space`] module documentation for accuracy notes.
//!
//! ## Quick start
//!
//! ```rust
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
//! ```
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `std` | Required (or `libm`) to use [`space`]: accurate gamma correction (`x^2.4`) and Oklch trig conversions need a math backend |
//! | `libm` | Alternative to `std` for [`space`] in `no_std` environments |
//! | `libm-arch` | Architecture-specific `libm` intrinsics; only meaningful with `libm` |
//! | `bytemuck` | Derives `Pod`/`Zeroable` for all pixel format types, enabling zero-copy buffer casting |
//! | `serde` | Derives `Serialize`/`Deserialize` for pixel format and color space types |
//! | `blend` | Enables the [`blend`] module (Porter-Duff compositing, premultiplied alpha) |
//!
//! No feature is enabled by default. The pixel-format layer (`rgb`, `gray`,
//! `alpha`) needs none of them. [`space`] needs `std` or `libm` — pick
//! whichever fits your target; `cargo build` will fail with a clear error if
//! you forget.
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
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(any(feature = "std", test))]
extern crate std;

pub mod alpha;
pub mod gray;
pub mod prelude;
pub mod rgb;

#[cfg(feature = "blend")]
pub mod blend;
pub mod named;
pub mod space;
