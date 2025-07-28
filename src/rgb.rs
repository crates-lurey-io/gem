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
//! let red = Abgr8888::new(128, 0, 0, 255);
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
//! let red: Rgb<u16> = Rgb::new(65535, 0, 0);
//! ```
//!
//! For additional types to store alpha channels (such as [`AlphaFirst`]), see [`crate::alpha`].

use crate::{
    alpha::{AlphaFirst, AlphaLast, HasAlpha},
    scalar::intensity::Intensity,
};

mod has_red;
pub use has_red::HasRed;

mod has_blue;
pub use has_blue::HasBlue;

mod has_green;
pub use has_green::HasGreen;

mod impl_rgba;

/// A trait for types that have red, green, and blue components.
pub trait RgbColor<T> {
    /// Creates a new color using the same value for each component.
    ///
    /// The resulting color is in grayscale.
    #[must_use]
    fn new_gray(value: T) -> Self
    where
        T: Copy,
        Self: Sized + Default + HasRed<T> + HasGreen<T> + HasBlue<T>,
    {
        let mut color = Self::default();
        color.set_red(value);
        color.set_green(value);
        color.set_blue(value);
        color
    }

    /// Creates a new color with the given red, green, and blue components.
    #[must_use]
    fn from_rgb(red: T, green: T, blue: T) -> Self
    where
        Self: Sized + Default + HasRed<T> + HasGreen<T> + HasBlue<T>,
    {
        let mut color = Self::default();
        color.set_red(red);
        color.set_green(green);
        color.set_blue(blue);
        color
    }

    /// Creates a new color with the given normalized 32-bit floating point red, green, and blue components.
    ///
    /// The numbers are clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn from_rgb_normalized_f32(red: f32, green: f32, blue: f32) -> Self
    where
        Self: Sized + Default + HasRed<T> + HasGreen<T> + HasBlue<T>,
        T: Intensity + Into<f32>,
    {
        let red = T::from_normalized_f32(red.clamp(0.0, 1.0));
        let green = T::from_normalized_f32(green.clamp(0.0, 1.0));
        let blue = T::from_normalized_f32(blue.clamp(0.0, 1.0));
        Self::from_rgb(red, green, blue)
    }

    /// Creates a new color with the given normalized 64-bit floating point red, green, and blue components.
    ///
    /// The numbers are clamped to the range [0.0, 1.0] before conversion.
    #[cfg(any(feature = "std", feature = "libm"))]
    #[must_use]
    fn from_rgb_normalized_f64(red: f64, green: f64, blue: f64) -> Self
    where
        Self: Sized + Default + HasRed<T> + HasGreen<T> + HasBlue<T>,
        T: Intensity + Into<f64>,
    {
        let red = T::from_normalized_f64(red.clamp(0.0, 1.0));
        let green = T::from_normalized_f64(green.clamp(0.0, 1.0));
        let blue = T::from_normalized_f64(blue.clamp(0.0, 1.0));
        Self::from_rgb(red, green, blue)
    }

    /// Returns the inner representation of the color as a tuple of red, green, and blue components.
    #[must_use]
    fn into_rgb(self) -> (T, T, T)
    where
        Self: Sized + HasRed<T> + HasGreen<T> + HasBlue<T>,
    {
        (self.red(), self.green(), self.blue())
    }
}

impl<T> RgbColor<T> for T where T: HasRed<T> + HasGreen<T> + HasBlue<T> {}

macro_rules! impl_rgb_with_fields {
    ($ty:ident<$t:ident>) => {
        impl<$t> HasRed<$t> for $ty<$t>
        where
            $t: Copy,
        {
            fn red(&self) -> $t {
                self.r
            }

            fn set_red(&mut self, value: $t) {
                self.r = value;
            }
        }

        impl<$t> HasGreen<$t> for $ty<$t>
        where
            $t: Copy,
        {
            fn green(&self) -> $t {
                self.g
            }

            fn set_green(&mut self, value: $t) {
                self.g = value;
            }
        }

        impl<$t> HasBlue<$t> for $ty<$t>
        where
            $t: Copy,
        {
            fn blue(&self) -> $t {
                self.b
            }

            fn set_blue(&mut self, value: $t) {
                self.b = value;
            }
        }
    };
}

macro_rules! impl_rgb_packed {
    (
        $ty:ident,
        red:  { shift: $rshift:expr, mask: $rmask:expr, clear: $rclear:expr },
        green:{ shift: $gshift:expr, mask: $gmask:expr, clear: $gclear:expr },
        blue: { shift: $bshift:expr, mask: $bmask:expr, clear: $bclear:expr }
    ) => {
        impl HasRed<u8> for $ty {
            fn red(&self) -> u8 {
                ((self.packed >> $rshift) & $rmask) as u8
            }
            fn set_red(&mut self, value: u8) {
                self.packed = (self.packed & $rclear) | ((u16::from(value) & $rmask) << $rshift);
            }
        }
        impl HasGreen<u8> for $ty {
            fn green(&self) -> u8 {
                ((self.packed >> $gshift) & $gmask) as u8
            }
            fn set_green(&mut self, value: u8) {
                self.packed = (self.packed & $gclear) | ((u16::from(value) & $gmask) << $gshift);
            }
        }
        impl HasBlue<u8> for $ty {
            fn blue(&self) -> u8 {
                ((self.packed >> $bshift) & $bmask) as u8
            }
            fn set_blue(&mut self, value: u8) {
                self.packed = (self.packed & $bclear) | ((u16::from(value) & $bmask) << $bshift);
            }
        }
    };
}

macro_rules! impl_with_alpha_packed {
    ($ty:ident, $alpha_shift:expr, $alpha_mask:expr, $alpha_clear:expr) => {
        impl HasAlpha<u8> for $ty {
            fn alpha(&self) -> u8 {
                ((self.packed >> $alpha_shift) & $alpha_mask) as u8
            }

            fn set_alpha(&mut self, value: u8) {
                self.packed = (self.packed & $alpha_clear)
                    | ((u16::from(value) & $alpha_mask) << $alpha_shift);
            }
        }
    };
}

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
    pub const fn new(red: T, green: T, blue: T) -> Self {
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

impl_rgb_with_fields!(Rgb<T>);

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
    /// Creates a new BGR color with the given blue, green, and red components.
    #[must_use]
    pub const fn new(blue: T, green: T, red: T) -> Self {
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

impl_rgb_with_fields!(Bgr<T>);

/// A 16-bit packed ARGB color representation.
///
/// Each component is represented by 4 bits, with the order being alpha, red, green, and blue.
///
/// ## Layout
///
/// ```c
/// struct Argb4444 {
///   uint16_t packed_argb;
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct Argb4444 {
    packed: u16,
}

impl Argb4444 {
    /// Creates a new ARGB color from the 16-bit packed representation.
    #[must_use]
    pub const fn new(packed: u16) -> Self {
        Self { packed }
    }
}

impl_rgb_packed!(
    Argb4444,
    red:   { shift: 8, mask: 0x0F, clear: 0xF0FF },
    green: { shift: 4, mask: 0x0F, clear: 0xFF0F },
    blue:  { shift: 0, mask: 0x0F, clear: 0xFFF0 }
);

impl_with_alpha_packed!(Argb4444, 12, 0x0F, 0x0FFF);

/// A 16-bit packed ARGB color representation.
///
/// Each component is represented by 1 bit for alpha, and 4 bits each for red, green, and blue.
///
/// ## Layout
///
/// ```c
/// struct Argb1555 {
///   uint16_t packed_argb;
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct Argb1555 {
    packed: u16,
}

impl_rgb_packed!(
    Argb1555,
    red:   { shift: 10, mask: 0x1F, clear: 0xFFE0 },
    green: { shift: 5, mask: 0x1F, clear: 0xFF9F },
    blue:  { shift: 0, mask: 0x1F, clear: 0xFFE0 }
);

impl_with_alpha_packed!(Argb1555, 15, 0x01, 0x7FFF);

/// A 16-bit packed RGB color representation.
///
/// Each component is represented by 5 bits for red, 6 bits for green, and 5 bits for blue.
///
/// ## Layout
///
/// ```c
/// struct Rgb565 {
///   uint16_t packed_rgb;
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct Rgb565 {
    packed: u16,
}

impl_rgb_packed!(
    Rgb565,
    red:   { shift: 11, mask: 0x1F, clear: 0xFFE0 },
    green: { shift: 5, mask: 0x3F, clear: 0xFF9F },
    blue:  { shift: 0, mask: 0x1F, clear: 0xFFE0 }
);

/// 8-bit RGB color representation.
///
/// Each component is represented by 8 bits, with 8 additional bits for padding.
///
/// ## Layout
///
/// ```c
/// struct Rgb888 {
///   uint8_t r;
///   uint8_t g;
///   uint8_t b;
/// }
/// ```
pub type Rgb888 = Rgb<u8>;

/// 8-bit ARGB color representation.
///
/// Each component is represented by 8 bits for alpha, red, green, and blue.
///
/// ## Layout
///
/// ```c
/// struct Argb8888 {
///   uint8_t a;
///   uint8_t r;
///   uint8_t g;
///   uint8_t b;
/// }
/// ```
pub type Argb8888 = AlphaFirst<u8, Rgb888>;

/// 8-bit BGR color representation.
///
/// Each component is represented by 8 bits, with the order being blue, green, and red.
///
/// ## Layout
///
/// ```c
/// struct Bgr888 {
///   uint8_t b;
///   uint8_t g;
///   uint8_t r;
/// }
/// ```
pub type Bgr888 = Bgr<u8>;

/// 8-bit ABGR color representation.
///
/// Each component is represented by 8 bits for alpha, blue, green, and red.
///
/// ## Layout
///
/// ```c
/// struct Abgr8888 {
///   uint8_t a;
///   uint8_t b;
///   uint8_t g;
///   uint8_t r;
/// }
/// ```
pub type Abgr8888 = AlphaFirst<u8, Bgr888>;

impl Abgr8888 {
    /// Creates a new ABGR color from the individual components.
    #[must_use]
    pub const fn new(a: u8, b: u8, g: u8, r: u8) -> Self {
        Self::with_color(a, Bgr::new(b, g, r))
    }
}

/// Floating-point RGB color representation.
///
/// Each component is represented by 32 bits (f32).
///
/// ## Layout
///
/// ```c
/// struct Rgbf32 {
///   float r;
///   float g;
///   float b;
/// }
/// ```
pub type Rgbf32 = Rgb<f32>;

/// Floating-point RGBA color representation.
///
/// Each component is represented by 32 bits (f32).
///
/// ## Layout
///
/// ```c
/// struct Rgbaf32 {
///   float r;
///   float g;
///   float b;
///   float a;
/// }
/// ```
pub type Rgbaf32 = AlphaLast<Rgbf32, f32>;

impl Rgbaf32 {
    /// Creates a new RGBA color from the individual components.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::with_color(Rgbf32::new(r, g, b), a)
    }
}
