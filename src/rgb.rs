//! Color representations that contain red, green, and blue components.
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
//! [`Abgr8888`] and [`RgbaF32`] are the most common formats, but there are many others:
//!
//! Type         | Bits per pixel | Description
//! ------------ | -------------- | -----------
//! [`Abgr8888`] | 32             | 8 bits each for alpha, blue, green, red
//! [`Argb1555`] | 16             | 5 bits each for RGB, 1 bit alpha
//! [`Argb4444`] | 16             | 4 bits each for RGB, 4 bits alpha
//! [`Argb8888`] | 32             | 8 bits each for alpha, red, green, blue
//! [`Bgr888`]   | 24             | 8 bits each for RGB, no padding (`size_of == 3`)
//! [`Rgb565`]   | 16             | 5 bits for red, 6 bits for green, 5 bits for blue
//! [`Rgb888`]   | 24             | 8 bits each for red, green, blue, no padding (`size_of == 3`)
//! [`RgbaF32`]  | 128            | 32 bits each for red, green, blue, alpha
//! [`RgbF32`]   | 96             | 32 bits each for red, green, blue
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
//! ## Arithmetic
//!
//! [`Mix`][crate::Mix] interpolates any two colors of the same format by an
//! `f32` factor. The 8-bit formats additionally carry `const fn` integer
//! operations that need no floating point at all, so they can run inside a
//! `const fn` color pipeline or a `no_std` blitter:
//!
//! ```rust
//! use gem::rgb::Rgb888;
//!
//! const GRASS: Rgb888 = Rgb888::from_rgb(200, 180, 60);
//! const SHADOWED: Rgb888 = GRASS.multiply(Rgb888::from_rgb(128, 128, 128));
//! assert_eq!(SHADOWED, Rgb888::from_rgb(100, 90, 30));
//! ```
//!
//! See [`Rgb888::multiply`], [`Rgb888::screen`], [`Rgb888::mix_u8`],
//! [`Rgb888::distance_sq`], and the scalar primitives they are built from in
//! [`crate::channel`].
//!
//! For additional types to store alpha channels (such as [`AlphaFirst`][]), see [`crate::alpha`].
//!
//! [`AlphaFirst`]: `crate::alpha::AlphaFirst`

mod formats;
mod impl_rgb_alpha_wrappers;
mod macros;
mod traits;

pub use formats::*;

/// Squared euclidean distance between two 8-bit RGB colors.
///
/// The result is at most `3 * 255^2 == 195_075`, so it always fits a `u32` and
/// cannot overflow. The square root is deliberately not taken: comparing
/// squared distances orders candidates identically and stays in integer math,
/// which is what a nearest-color palette search actually needs.
///
/// This is a plain euclidean distance in gamma-encoded sRGB, which is fast but
/// not perceptually uniform. For perceptual nearest-color matching, convert to
/// [`Oklab`][crate::space::Oklab] and use
/// [`Oklab::distance_sq`][crate::space::Oklab::distance_sq] instead.
///
/// Takes tuples rather than a color type so callers holding raw channels (a
/// shader uniform, an FFI struct, a packed `u32`) need no conversion; for gem's
/// own types see [`Rgb888::distance_sq`] and [`Bgr888::distance_sq`].
///
/// ## Examples
///
/// ```rust
/// use gem::rgb::distance_sq;
///
/// assert_eq!(distance_sq((0, 0, 0), (0, 0, 0)), 0);
/// assert_eq!(distance_sq((0, 0, 0), (255, 255, 255)), 195_075);
///
/// // Symmetric, and usable in a `const` context.
/// const D: u32 = distance_sq((10, 20, 30), (13, 24, 30));
/// assert_eq!(D, 25);
/// ```
#[must_use]
pub const fn distance_sq(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let dr = a.0.abs_diff(b.0) as u32;
    let dg = a.1.abs_diff(b.1) as u32;
    let db = a.2.abs_diff(b.2) as u32;
    dr * dr + dg * dg + db * db
}
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", allow(clippy::unsafe_derive_deserialize))]
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

    /// Returns the components as an `(r, g, b)` tuple.
    ///
    /// This is the `const` counterpart of [`RgbColor::to_rgb`], which is a
    /// trait method and therefore not callable in a `const` context on stable
    /// Rust. Prefer this one when writing `const fn` color pipelines.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// const RED: (u8, u8, u8) = Rgb888::from_rgb(255, 0, 0).to_rgb();
    /// assert_eq!(RED, (255, 0, 0));
    /// ```
    #[must_use]
    pub const fn to_rgb(self) -> (T, T, T)
    where
        T: Copy,
    {
        (self.r, self.g, self.b)
    }
}

#[cfg(feature = "bytemuck")]
#[allow(unsafe_code)]
unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for Rgb<T> {}

#[cfg(feature = "bytemuck")]
#[allow(unsafe_code)]
unsafe impl<T: bytemuck::Pod> bytemuck::Pod for Rgb<T> {}

macros::impl_rgb_with_fields!(Rgb<T>);

#[cfg(feature = "space")]
impl<T> crate::space::RgbChannelScale for Rgb<T>
where
    T: crate::space::NativeMax,
{
    const RED_MAX: f32 = T::MAX;
    const GREEN_MAX: f32 = T::MAX;
    const BLUE_MAX: f32 = T::MAX;
}

#[cfg(feature = "space")]
impl<T> From<crate::space::Srgb> for Rgb<T>
where
    T: crate::space::NativeMax,
{
    fn from(c: crate::space::Srgb) -> Self {
        crate::space::FromSrgb::from_srgb(c)
    }
}

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", allow(clippy::unsafe_derive_deserialize))]
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

    /// Returns the components as a `(b, g, r)` tuple, in memory order.
    ///
    /// Note the order: this mirrors [`from_bgr`][Self::from_bgr], not
    /// [`Rgb::to_rgb`]. For the color-ordered tuple, use
    /// [`to_rgb`][Self::to_rgb].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// const RED: (u8, u8, u8) = Bgr888::from_bgr(0, 0, 255).to_bgr();
    /// assert_eq!(RED, (0, 0, 255));
    /// ```
    #[must_use]
    pub const fn to_bgr(self) -> (T, T, T)
    where
        T: Copy,
    {
        (self.b, self.g, self.r)
    }

    /// Returns the components as an `(r, g, b)` tuple, in color order.
    ///
    /// The `const` counterpart of [`RgbColor::to_rgb`]. Storage order does not
    /// affect the result: a `Bgr888` and an `Rgb888` of the same color return
    /// the same tuple, which is what makes it safe to pass across a channel-math
    /// boundary that only knows about `(r, g, b)`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::{Bgr888, Rgb888};
    ///
    /// assert_eq!(
    ///     Bgr888::from_bgr(0, 0, 255).to_rgb(),
    ///     Rgb888::from_rgb(255, 0, 0).to_rgb(),
    /// );
    /// ```
    #[must_use]
    pub const fn to_rgb(self) -> (T, T, T)
    where
        T: Copy,
    {
        (self.r, self.g, self.b)
    }
}

#[cfg(feature = "bytemuck")]
#[allow(unsafe_code)]
unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for Bgr<T> {}

#[cfg(feature = "bytemuck")]
#[allow(unsafe_code)]
unsafe impl<T: bytemuck::Pod> bytemuck::Pod for Bgr<T> {}

macros::impl_rgb_with_fields!(Bgr<T>);

#[cfg(feature = "space")]
impl<T> crate::space::RgbChannelScale for Bgr<T>
where
    T: crate::space::NativeMax,
{
    const RED_MAX: f32 = T::MAX;
    const GREEN_MAX: f32 = T::MAX;
    const BLUE_MAX: f32 = T::MAX;
}

#[cfg(feature = "space")]
impl<T> From<crate::space::Srgb> for Bgr<T>
where
    T: crate::space::NativeMax,
{
    fn from(c: crate::space::Srgb) -> Self {
        crate::space::FromSrgb::from_srgb(c)
    }
}
