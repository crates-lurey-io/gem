//! Color representations that contain red, green, and blue components.

use crate::alpha::{AlphaFirst, AlphaLast};

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

/// A color representation that contains alpha, red, green, and blue components.
///
/// ## Layout
///
/// ```c
/// template<typename T>
/// struct Argb {
///   T a;
///   T r;
///   T g;
///   T b;
/// }
/// ```
pub type Argb<T> = AlphaFirst<T, Rgb<T>>;

/// A color representation that contains red, green, blue, and alpha components.
///
/// ## Layout
///
/// ```c
/// template<typename T>
/// struct Rgba {
///   T r;
///   T g;
///   T b;
///   T a;
/// }
/// ```
pub type Rgba<T> = AlphaLast<Rgb<T>, T>;

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
#[repr(transparent)]
pub struct Argb4444 {
    packed: u16,
}

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
#[repr(transparent)]
pub struct Argb1555 {
    packed: u16,
}

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
#[repr(transparent)]
pub struct Rgb565 {
    packed: u16,
}

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Bgr888 {
    b: u8,
    g: u8,
    r: u8,
}

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
