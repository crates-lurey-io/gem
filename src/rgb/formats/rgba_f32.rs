use crate::{alpha::AlphaLast, rgb::Rgbf32};

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
///
/// ## Examples
///
/// To create an `Rgbaf32` color from individual components:
///
/// ```rust
/// use gem::rgb::Rgbaf32;
///
/// let color = Rgbaf32::from_rgba(1.0, 0.0, 0.0, 1.0);
/// ```
pub type Rgbaf32 = AlphaLast<f32, Rgbf32>;

impl Rgbaf32 {
    /// Creates a new RGBA color from the individual components.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::{alpha::HasAlpha, rgb::{HasRed, HasGreen, HasBlue, Rgbaf32}};
    ///
    /// let color = Rgbaf32::from_rgba(1.0, 0.0, 0.0, 1.0);
    /// assert_eq!(color.red(), 1.0);
    /// assert_eq!(color.green(), 0.0);
    /// assert_eq!(color.blue(), 0.0);
    /// assert_eq!(color.alpha(), 1.0);
    /// ```
    #[must_use]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::with_color(a, Rgbf32::from_rgb(r, g, b))
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    use crate::{rgb::HasBlue, rgb::HasGreen, rgb::HasRed};

    #[test]
    fn test_new() {
        let color = Rgbaf32::from_rgba(1.0, 0.0, 0.0, 1.0);
        assert_eq!(color.red(), 1.0);
        assert_eq!(color.green(), 0.0);
        assert_eq!(color.blue(), 0.0);
        assert_eq!(color.alpha(), 1.0);
    }
}
