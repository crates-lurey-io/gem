use core::ops::{Deref, DerefMut};

/// Alpha-only color type.
///
/// ## Layout
///
/// The layout of this type is always the same as the underlying type `T` (`#[repr(transparent)]`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Alpha<T> {
    alpha: T,
}

impl<T> Alpha<T> {
    /// Creates a new instance of `Alpha` with the given alpha value.
    #[must_use]
    pub const fn new(alpha: T) -> Self {
        Self { alpha }
    }

    /// Returns the alpha value of this color.
    #[must_use]
    pub const fn alpha(&self) -> T
    where
        T: Copy,
    {
        self.alpha
    }

    /// Consumes and returns the alpha value of this color.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.alpha
    }
}

/// 8-bit alpha-only color type.
///
/// ## Layout
///
/// This type has the same layout as `u8`.
pub type Alpha8 = Alpha<u8>;

/// A contingous representation of a color with alpha first, followed by other color components.
///
/// ## Layout
///
/// ```c
/// template<typename A, typename C>
/// struct AlphaFirst {
///   A alpha;
///   C color;
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct AlphaFirst<A, C> {
    alpha: A,
    color: C,
}

impl<A, C> AlphaFirst<A, C> {
    /// Creates a new instance of `AlphaFirst` with the given alpha and color components.
    #[must_use]
    pub const fn with_color(alpha: A, color: C) -> Self {
        Self { alpha, color }
    }

    /// Returns the alpha component of this color.
    #[must_use]
    pub const fn alpha(&self) -> A
    where
        A: Copy,
    {
        self.alpha
    }

    /// Returns the color component of this color.
    #[must_use]
    pub const fn color(&self) -> C
    where
        C: Copy,
    {
        self.color
    }

    /// Consumes and returns the alpha and color components of this color.
    #[must_use]
    pub fn into_inner(self) -> (A, C) {
        (self.alpha, self.color)
    }
}

impl<A, C> Deref for AlphaFirst<A, C>
where
    C: Copy,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.color
    }
}

impl<A, C> DerefMut for AlphaFirst<A, C>
where
    C: Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.color
    }
}

/// A contingous representation of a color with alpha last, preceded by other color components.
///
/// ## Layout
///
/// ```c
/// template<typename C, typename A>
/// struct AlphaLast {
///   C color;
///   A alpha;
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct AlphaLast<A, C> {
    color: C,
    alpha: A,
}

impl<A, C> AlphaLast<A, C> {
    /// Creates a new instance of `AlphaFirst` with the given color and alpha components.
    #[must_use]
    pub const fn with_color(alpha: A, color: C) -> Self {
        Self { color, alpha }
    }

    /// Returns the alpha component of this color.
    #[must_use]
    pub const fn alpha(&self) -> A
    where
        A: Copy,
    {
        self.alpha
    }

    /// Returns the color component of this color.
    #[must_use]
    pub const fn color(&self) -> C
    where
        C: Copy,
    {
        self.color
    }

    /// Consumes and returns the color and alpha components of this color.
    #[must_use]
    pub fn into_inner(self) -> (C, A) {
        (self.color, self.alpha)
    }
}

impl<A, C> Deref for AlphaLast<A, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.color
    }
}

impl<A, C> DerefMut for AlphaLast<A, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_is_repr_transparent() {
        let alpha = Alpha::new(128u8);
        let bytes = unsafe { core::mem::transmute::<Alpha<u8>, [u8; 1]>(alpha) };
        assert_eq!(bytes, [128]);
    }

    #[test]
    fn alpha_new() {
        let alpha = Alpha::new(128u8);
        assert_eq!(alpha.alpha(), 128);
    }

    #[test]
    fn alpha_into_inner() {
        let alpha = Alpha::new(255u8);
        assert_eq!(alpha.into_inner(), 255);
    }

    #[test]
    fn alpha8_is_repr_transparent_u8() {
        let alpha8 = Alpha8::new(128);
        let bytes = unsafe { core::mem::transmute::<Alpha8, [u8; 1]>(alpha8) };
        assert_eq!(bytes, [128]);
    }

    #[test]
    fn alpha_first_new() {
        let alpha_first = AlphaFirst::with_color(128u8, [255, 0, 0]);
        assert_eq!(alpha_first.alpha(), 128);
        assert_eq!(alpha_first.color(), [255, 0, 0]);
    }

    #[test]
    fn alpha_first_into_inner() {
        let alpha_first = AlphaFirst::with_color(128u8, [255, 0, 0]);
        assert_eq!(alpha_first.into_inner(), (128, [255, 0, 0]));
    }

    #[test]
    fn alpha_first_deref_color() {
        let alpha_first = AlphaFirst::with_color(128u8, [255, 0, 0]);
        assert_eq!(*alpha_first, [255, 0, 0]);
    }

    #[test]
    fn alpha_first_deref_mut_color() {
        let mut alpha_first = AlphaFirst::with_color(128u8, [255, 0, 0]);
        alpha_first.color = [0, 255, 0];
        assert_eq!(*alpha_first, [0, 255, 0]);
    }

    #[test]
    fn alpha_first_repr_c() {
        let alpha_first = AlphaFirst::with_color(128u8, [255, 0, 0]);
        let bytes =
            unsafe { core::mem::transmute::<AlphaFirst<u8, [u8; 3]>, [u8; 4]>(alpha_first) };
        assert_eq!(bytes, [128, 255, 0, 0]);
    }

    #[test]
    fn alpha_last_new() {
        let alpha_last = AlphaLast::with_color(128u8, [255, 0, 0]);
        assert_eq!(alpha_last.alpha(), 128);
        assert_eq!(alpha_last.color(), [255, 0, 0]);
    }

    #[test]
    fn alpha_last_into_inner() {
        let alpha_last = AlphaLast::with_color(128u8, [255, 0, 0]);
        assert_eq!(alpha_last.into_inner(), ([255, 0, 0], 128));
    }

    #[test]
    fn alpha_last_deref_color() {
        let alpha_last = AlphaLast::with_color(128u8, [255, 0, 0]);
        assert_eq!(*alpha_last, [255, 0, 0]);
    }

    #[test]
    fn alpha_last_deref_mut_color() {
        let mut alpha_last = AlphaLast::with_color(128u8, [255, 0, 0]);
        alpha_last.color = [0, 255, 0];
        assert_eq!(*alpha_last, [0, 255, 0]);
    }

    #[test]
    fn alpha_last_repr_c() {
        let alpha_last = AlphaLast::with_color(128u8, [255, 0, 0]);
        let bytes = unsafe { core::mem::transmute::<AlphaLast<u8, [u8; 3]>, [u8; 4]>(alpha_last) };
        assert_eq!(bytes, [255, 0, 0, 128]);
    }
}
