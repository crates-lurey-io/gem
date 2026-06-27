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
//! | `bytemuck` | Derives `Pod`/`Zeroable` for all pixel format types, enabling zero-copy buffer casting |
//! | `std` | Enables accurate gamma correction (`x^2.4`) and Oklch trig conversions |

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
