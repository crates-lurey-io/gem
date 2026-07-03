//! Common types and traits for introductory use.
//!
//! ## Examples
//!
//! For RGB pixel formats:
//!
//! ```rust
//! use gem::prelude::*;
//!
//! // 8-bit RGB color
//! let red = Rgb888::from_rgb(255, 0, 0);
//!
//! // 32-bit floating point RGB color
//! let red = RgbF32::from_rgb(1.0, 0.0, 0.0);
//! ```
//!
//! For color space manipulation:
//!
//! ```rust
//! use gem::prelude::*;
//!
//! // Convert a pixel to HSL and lighten it
//! let pixel = Rgb888::from_rgb(200, 50, 100);
//! let hsl = Hsl::from(Srgb::from(pixel));
//! let lighter: Rgb888 = Srgb::from(hsl.lighten(0.15)).into();
//! ```

pub use crate::{
    alpha::{HasAlpha as _, WithAlpha as _},
    gray::HasGray as _,
    rgb::{
        Abgr8888, HasBlue as _, HasGreen as _, HasRed as _, Lerp as _, Rgb888, RgbColor as _,
        RgbaF32, RgbF32,
    },
    space::{Hsl, Hsv, Oklab, Oklch, Srgb},
};
