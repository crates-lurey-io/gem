//! Color representations and conversions.
//!
//! ## Features
//!
//! By default, this crate uses `#![no_std]` and does not depend on the standard library.
//!
//! ### `bytemuck`
//!
//! Derives `bytemuck::Zeroable` and `bytemuck::Pod` for color types.
//!
//! ### `libm`
//!
//! _Enabled by default._
//!
//! This feature enables the use of the `libm` crate for mathematical operations.
//!
//! - If both `libm` and `std` features are enabled, `std` will be used for math operations.
//! - If neither `libm` nor `std` is enabled, a fallback implementation will be used.
//!
//! ### `libm-arch`
//!
//! _Enabled by default._
//!
//! This feature enables architecture-specific optimizations for the `libm` crate.
//!
//! ### `std`
//!
//! Uses the standard library instead of `libm` for mathematical operations.
//!
//! - If both `libm` and `std` features are enabled, `std` will be used for math operations.
//! - If neither `libm` nor `std` is enabled, a fallback implementation will be used.

#![no_std]

pub mod alpha;
pub mod gray;
pub mod prelude;
pub mod rgb;
pub mod scalar;

pub(crate) mod internal;
