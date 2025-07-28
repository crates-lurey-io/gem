//! Color representations and conversions.

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
