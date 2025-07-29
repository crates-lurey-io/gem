//! Color representations and conversions.
//!
//! ## Features
//!
//! By default, this crate uses `#![no_std]` and does not depend on the standard library.
//!
//! ### `bytemuck`
//!
//! Derives `bytemuck::Zeroable` and `bytemuck::Pod` for color types.

#![no_std]

pub mod alpha;
pub mod gray;
pub mod prelude;
pub mod rgb;
