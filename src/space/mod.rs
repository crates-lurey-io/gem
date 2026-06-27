//! Perceptual and working color spaces.
//!
//! This module provides color space types for manipulation, conversion, and
//! interpolation — as opposed to the pixel format types in [`crate::rgb`] and
//! [`crate::gray`], which are optimized for memory layout and GPU interop.
//!
//! ## Choosing a color space
//!
//! | Space | Use when |
//! |-------|----------|
//! | [`Srgb`] | Reading/writing CSS colors, PNG pixels, interop with other libs |
//! | [`LinearRgb`] | Physically-correct light mixing and blending |
//! | [`Hsl`] | Artist-friendly manipulation: lighten, darken, saturate, complement |
//! | [`Hsv`] | Color pickers; same hue as HSL but different brightness model |
//! | [`Oklab`] | Perceptually-uniform blending, gradients, color difference |
//! | [`Oklch`] | Perceptually-uniform hue rotation and theme generation |
//!
//! ## Conversion chain
//!
//! All types implement `From`/`Into` for their neighbors:
//!
//! ```text
//! Rgb888 ──► Srgb ◄──► Hsl
//!              │  ◄──► Hsv
//!              │
//!              ▼
//!         LinearRgb ◄──► Oklab ◄──► Oklch   (Oklch requires `std`)
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use gem::space::{Hsl, Srgb};
//! use gem::rgb::{HasGreen as _, HasRed as _, Rgb888};
//!
//! // Lighten a pixel color
//! let pixel = Rgb888::from_rgb(200, 50, 100);
//! let hsl = Hsl::from(Srgb::from(pixel));
//! let lighter: Rgb888 = Srgb::from(hsl.lighten(0.15)).into();
//! assert!(lighter.red() >= pixel.red() || lighter.green() >= pixel.green());
//! ```

mod math;

mod srgb;
mod linear;
mod hsl;
mod hsv;
mod oklab;
mod oklch;
mod convert;

pub use srgb::Srgb;
pub use linear::LinearRgb;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use oklab::Oklab;
pub use oklch::Oklch;
