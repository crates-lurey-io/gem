use crate::rgb::Rgb;

/// Floating-point RGB color representation.
///
/// Each component is represented by 32 bits (f32).
///
/// ## Layout
///
/// ```c
/// struct RgbF32 {
///   float r;
///   float g;
///   float b;
/// }
/// ```
///
/// ## Examples
///
/// To create an `RgbF32` color from individual components:
///
/// ```rust
/// use gem::rgb::RgbF32;
///
/// let color = RgbF32::from_rgb(1.0, 0.0, 0.0);
/// ```
pub type RgbF32 = Rgb<f32>;

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    use crate::{rgb::HasBlue, rgb::HasGreen, rgb::HasRed};

    #[test]
    fn test_new() {
        let color = RgbF32::from_rgb(1.0, 0.0, 0.0);
        assert_eq!(color.red(), 1.0);
        assert_eq!(color.green(), 0.0);
        assert_eq!(color.blue(), 0.0);
    }
}
