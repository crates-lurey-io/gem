use crate::alpha::{AlphaFirst, AlphaLast};

/// Attaches an alpha channel to any color, generically.
///
/// Before this trait, adding an alpha channel to a packed 16-bit format
/// (e.g. [`Argb1555`][crate::rgb::Argb1555]) and a struct-of-fields format
/// (e.g. [`Rgb888`][crate::rgb::Rgb888]) were two unrelated mechanisms: the
/// former baked alpha into the packed bits via
/// `crate::rgb::macros::impl_with_alpha_packed!`, the latter required
/// manually reaching for [`AlphaFirst`] or [`AlphaLast`]. `WithAlpha` is a
/// single, uniform way to combine *any* `Copy` color (RGB, grayscale, or
/// otherwise) with an alpha channel of any type — including types outside
/// this crate.
///
/// ## Examples
///
/// ```rust
/// use gem::{alpha::{HasAlpha, WithAlpha}, rgb::{HasRed, Rgb888}};
///
/// let opaque = Rgb888::from_rgb(255, 0, 0);
/// let translucent = opaque.with_alpha_last(128u8);
/// assert_eq!(translucent.red(), 255);
/// assert_eq!(translucent.alpha(), 128);
/// ```
pub trait WithAlpha: Sized + Copy {
    /// Wraps `self` with `alpha`, storing alpha **before** the color in memory
    /// (see [`AlphaFirst`]).
    #[must_use]
    fn with_alpha_first<A>(self, alpha: A) -> AlphaFirst<A, Self> {
        AlphaFirst::with_color(alpha, self)
    }

    /// Wraps `self` with `alpha`, storing alpha **after** the color in memory
    /// (see [`AlphaLast`]).
    #[must_use]
    fn with_alpha_last<A>(self, alpha: A) -> AlphaLast<A, Self> {
        AlphaLast::with_color(alpha, self)
    }
}

impl<C> WithAlpha for C where C: Copy {}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::{
        gray::Gray8,
        rgb::{HasBlue, HasGreen, HasRed, Rgb888},
    };

    #[test]
    fn rgb888_with_alpha_last() {
        let color = Rgb888::from_rgb(1, 2, 3).with_alpha_last(200u8);
        assert_eq!((color.red(), color.green(), color.blue()), (1, 2, 3));
        assert_eq!(color.alpha(), 200);
    }

    #[test]
    fn rgb888_with_alpha_first() {
        let color = Rgb888::from_rgb(1, 2, 3).with_alpha_first(200u8);
        assert_eq!((color.red(), color.green(), color.blue()), (1, 2, 3));
        assert_eq!(color.alpha(), 200);
    }

    #[test]
    fn works_for_non_rgb_colors_too() {
        // Gray8 has no dedicated alpha wrapper constant of its own (unlike
        // GrayAlpha8), but WithAlpha works for it uniformly anyway.
        let color = Gray8::new(128).with_alpha_last(64u8);
        assert_eq!(color.alpha(), 64);
    }

    #[test]
    fn alpha_type_need_not_match_color_component_type() {
        let color = Rgb888::from_rgb(1, 2, 3).with_alpha_last(0.5f32);
        assert_eq!(color.alpha(), 0.5);
    }
}
