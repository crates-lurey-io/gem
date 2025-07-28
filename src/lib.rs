//! Color representations and conversions.
//!
//! ## Features
//!
//! By default, this crate uses `#![no_std]` and does not depend on the standard library.
//!
//! At least one `libm` or `std` feature must be enabled to use floating-point operations.
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
//! ## Unstable Features
//!
//! ### `unstable-scalar`
//!
//! Enables the use of unstable scalar types for color representations.
//!
//! This feature is not recommended for production use as it may change in future versions.

#![no_std]

pub mod alpha;
pub mod gray;
pub mod prelude;
pub mod rgb;

#[cfg(not(feature = "unstable-scalar"))]
pub(crate) mod scalar;
#[cfg(feature = "unstable-scalar")]
pub mod scalar;

pub(crate) mod math;
