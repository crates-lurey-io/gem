//! Common types and traits for introductory use of the `gem` crate.
//!
//! ## Examples
//!
//! For RGB colors without an alpha channel:
//!
//! ```rust
//! use gem::prelude::*;
//!
//! // 8-bit RGB color
//! let red = Rgb888::new(255, 0, 0);
//!
//! // 32-bit floating point RGB color
//! let red = Rgbf32::new(1.0, 0.0, 0.0);
//! ```
//!
//! Or, with an alpha channel:
//!
//! ```rust
//! use gem::prelude::*;
//!
//! // 8-bit ARGB color, 50% transparent
//! let red = Abgr8888::new(128, 255, 0, 0);
//!
//! // 32-bit floating point RGB color with alpha, 50% transparent
//! let red = Rgbaf32::new(1.0, 0.0, 0.0, 0.5);
//! ```

pub use crate::rgb::{
    Abgr8888, Rgb888, RgbColor as _, Rgbaf32, Rgbf32, WithBlue as _, WithGreen as _, WithRed as _,
};
pub use crate::scalar::normalize::{Normalize as _, Normalized as _};
