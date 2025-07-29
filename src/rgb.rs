//! Color representations that contain 🔴 red, 🟢 green, and 🔵 blue components.
//!
//! This module contains:
//!
//! - Traits that work on all RGB data types
//! - Generic types that can be used to define RGB data types
//! - Concrete types that define common RGB data types
//!
//! ## Getting Started
//!
//! If this is your first time using this crate, consider [`crate::prelude`].
//!
//! ```rust
//! // The most common RGB type, used in OpenGL, Vulkan, and web technologies.
//! use gem::rgb::Abgr8888;
//!
//! // 8-bit ARGB color, with alpha first.
//! let red = Abgr8888::from_abgr(128, 0, 0, 255);
//! ```
//!
//! ## Predefined Types
//!
//! [`Abgr8888`] and [`Rgbaf32`] are the most common formats, but there are many others:
//!
//! Type         | Bits per pixel | Description
//! ------------ | -------------- | -----------
//! [`Abgr8888`] | 32             | 8 bits each for alpha, blue, green, red
//! [`Argb1555`] | 16             | 5 bits each for RGB, 1 bit alpha
//! [`Argb4444`] | 16             | 4 bits each for RGB, 4 bits alpha
//! [`Argb8888`] | 32             | 8 bits each for alpha, red, green, blue
//! [`Bgr888`]   | 24 (32 padded) | 8 bits each for RGB, 8 bits padding in memory
//! [`Rgb565`]   | 16             | 5 bits for red, 6 bits for green, 5 bits for blue
//! [`Rgb888`]   | 24 (32 padded) | 8 bits each for red, green, blue, 8 bits padding in memory
//! [`Rgbaf32`]  | 128            | 32 bits each for red, green, blue, alpha
//! [`Rgbf32`]   | 96             | 32 bits each for red, green, blue
//!
//! ## Generic Types
//!
//! Structs with generic types allow easily creating custom RGB types:
//!
//! - [`Rgb<T>`]; a generic RGB color representation stored in order of red, green, blue.
//! - [`Bgr<T>`]; a generic BGR color representation stored in order of blue, green, red.
//!
//! For example, to create an RGB color that stores each channel as a 16-bit integer:
//!
//! ```rust
//! use gem::rgb::Rgb;
//!
//! let red: Rgb<u16> = Rgb::from_rgb(65535, 0, 0);
//! ```
//!
//! For additional types to store alpha channels (such as [`AlphaFirst`][]), see [`crate::alpha`].
//!
//! [`AlphaFirst`]: `crate::alpha::AlphaFirst`

mod formats;
mod impl_rgb_alpha_wrappers;
mod macros;
mod traits;

pub use formats::*;
pub use traits::*;
pub use traits::{HasBlue as _, HasGreen as _, HasRed as _, RgbColor as _, RgbaColor as _};

/// A color representation that contains red, green, and blue components.
///
/// ## Layout
///
/// ```c
/// template<typename T>
/// struct Rgb {
///     T r;
///     T g;
///     T b;
/// };
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rgb<T> {
    r: T,
    g: T,
    b: T,
}

impl<T> Rgb<T> {
    /// Creates a new RGB color with the given red, green, and blue components.
    #[must_use]
    pub const fn from_rgb(red: T, green: T, blue: T) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for Rgb<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: bytemuck::Pod> bytemuck::Pod for Rgb<T> where T: bytemuck::Pod {}

macros::impl_rgb_with_fields!(Rgb<T>);

/// A color representation that contains blue, green, and red components.
///
/// ## Layout
///
/// ```c
/// template<typename T>
/// struct Bgr {
///     T b;
///     T g;
///     T r;
/// };
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Bgr<T> {
    b: T,
    g: T,
    r: T,
}

impl<T> Bgr<T> {
    /// Creates a new BGR color with the individual component values (b, g, r).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::{HasBlue, HasGreen, HasRed, Bgr};
    ///
    /// let color = Bgr::from_bgr(0, 255, 0);
    /// assert_eq!(color.blue(), 0);
    /// assert_eq!(color.green(), 255);
    /// assert_eq!(color.red(), 0);
    #[must_use]
    pub const fn from_bgr(blue: T, green: T, red: T) -> Self {
        Self {
            b: blue,
            g: green,
            r: red,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for Bgr<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: bytemuck::Pod> bytemuck::Pod for Bgr<T> where T: bytemuck::Pod {}

macros::impl_rgb_with_fields!(Bgr<T>);
