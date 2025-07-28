//! Color representations for ⚫ ⚪ grayscale colors.
//!
//! This module contains:
//!
//! - Traits that work on all Grayscale data types
//! - Generic types that can be used to define Grayscole data types
//! - Concrete types that define common Grayscale data types
//!
//! ## Getting Started
//!
//! ```rust
//! use gem::gray::Gray8;
//!
//! // 8-bit grayscale color
//! let gray = Gray8::new(128);
//!
//! // Or, with an alpha channel
//! use gem::gray::GrayAlpha8;
//!
//! // 8-bit grayscale color with alpha
//! let gray_alpha = GrayAlpha8::new(128, 255);
//! ```
//!
//! ## Predefined Types
//!
//! Type             | Bits per pixel | Description
//! ---------------- | -------------- | -----------
//! [`Gray8`]        | 8              | 8-bit grayscale color
//! [`Gray16`]       | 16             | 16-bit grayscale color
//! [`GrayF32`]      | 32             | 32-bit floating-point grayscale color
//! [`GrayAlpha8`]   | 16             | 8-bit grayscale color with alpha
//! [`GrayAlpha16`]  | 32             | 16-bit grayscale color with alpha
//! [`GrayAlphaF32`] | 64             | 32-bit floating-point grayscale color with alpha
//!
//! ## Generic Types
//!
//! Structs with generic types allow easily creating custom Grayscale types:
//!
//! - [`Gray<T>`]; a generic Grayscale color representation with a single component
//! - [`GrayAlpha<T>`]; a generic Grayscale color representation with an alpha channel

use crate::alpha::AlphaLast;

mod with_gray;
pub use with_gray::WithGray;

/// Grayscale-only color type.
///
/// ## Layout
///
/// The layout of this type is always the same as the underlying type `T` (`#[repr(transparent)]`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Gray<T> {
    gray: T,
}

impl<T> Gray<T> {
    /// Creates a new instance of `Gray` with the given gray value.
    #[must_use]
    pub const fn new(gray: T) -> Self {
        Self { gray }
    }

    /// Returns the gray value of this color.
    #[must_use]
    pub const fn gray(&self) -> T
    where
        T: Copy,
    {
        self.gray
    }
}

impl<T> WithGray<T> for Gray<T>
where
    T: Copy,
{
    fn gray(&self) -> T {
        self.gray
    }

    fn set_gray(&mut self, value: T) {
        self.gray = value;
    }
}

/// 8-bit grayscale-only color type.
///
/// ## Layout
///
/// This type has the same layout as [`u8`].
pub type Gray8 = Gray<u8>;

/// 16-bit grayscale-only color type.
///
/// ## Layout
///
/// This type has the same layout as [`u16`].
pub type Gray16 = Gray<u16>;

/// Combined grayscale and alpha color type.
///
/// ## Layout
///
/// ```c
/// template<typename T>
/// struct GrayAlpha {
///   T gray;
///   T alpha;
/// }
/// ```
pub type GrayAlpha<T> = AlphaLast<T, Gray<T>>;

impl<T> WithGray<T> for GrayAlpha<T>
where
    T: Copy,
{
    fn gray(&self) -> T {
        self.color().gray()
    }

    fn set_gray(&mut self, value: T) {
        let color = &mut **self;
        color.set_gray(value);
    }
}

/// 16-bit grayscale and alpha color type.
///
/// ## Layout
///
/// ```c
/// struct GrayAlpha8 {
///   uint8_t gray;
///   uint8_t alpha;
/// }
/// ```
pub type GrayAlpha8 = GrayAlpha<u8>;

impl GrayAlpha8 {
    /// Creates a new 8-bit grayscale color with alpha.
    #[must_use]
    pub const fn new(gray: u8, alpha: u8) -> Self {
        Self::with_color(alpha, Gray8::new(gray))
    }
}

/// 32-bit grayscale and alpha color type.
///
/// ## Layout
///
/// ```c
/// struct GrayAlpha16 {
///   uint16_t gray;
///   uint16_t alpha;
/// }
/// ```
pub type GrayAlpha16 = GrayAlpha<u16>;

impl GrayAlpha16 {
    /// Creates a new 16-bit grayscale color with alpha.
    #[must_use]
    pub const fn new(gray: u16, alpha: u16) -> Self {
        Self::with_color(alpha, Gray16::new(gray))
    }
}

/// 32-bit floating-point grayscale color type.
///
/// ## Layout
///
/// This type has the same layout as [`f32`].
pub type GrayF32 = Gray<f32>;

/// 32-bit floating-point grayscale and alpha color type.
///
/// ## Layout
///
/// ```c
/// struct GrayAlphaF32 {
///   float gray;
///   float alpha;
/// }
/// ```
pub type GrayAlphaF32 = AlphaLast<f32, GrayF32>;

impl GrayAlphaF32 {
    /// Creates a new 32-bit floating-point grayscale color with alpha.
    #[must_use]
    pub const fn new(gray: f32, alpha: f32) -> Self {
        Self::with_color(alpha, Gray::new(gray))
    }
}
