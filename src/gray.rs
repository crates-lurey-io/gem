use crate::alpha::AlphaLast;

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

/// 8-bit grayscale-only color type.
///
/// ## Layout
///
/// This type has the same layout as `u8`.
pub type Gray8 = Gray<u8>;

/// 16-bit grayscale-only color type.
///
/// ## Layout
///
/// This type has the same layout as `u16`.
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
pub type GrayAlpha<T> = AlphaLast<T, T>;

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
pub type GrayAlpha8 = GrayAlpha<Gray8>;

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
pub type GrayAlpha16 = GrayAlpha<Gray16>;

/// Floating-point grayscale color type.
///
/// ## Layout
///
/// This type has the same layout as `f32`.
pub type GrayF32 = Gray<f32>;
