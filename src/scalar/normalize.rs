use core::ops::Div;

use crate::scalar::intensity::Intensity;

/// A trait that represents scalar data types that can be normalized.
pub trait Normalize<N: Normalized> {
    /// Converts this scalar value to its normalized form.
    ///
    /// If the value cannot be normalized, it returns `None`.
    fn normalize(&self) -> Option<N>;

    /// Converts this scalar value to its normalized form, clamping it to the legal range.
    fn normalize_clamped(&self) -> N;

    /// Converts this scalar value to its normalized form without checking the input.
    ///
    /// ## Safety
    /// If the value is not within the legal range, this will lead to undefined behavior.
    unsafe fn normalize_unchecked(&self) -> N;
}

fn to_float_intensity<T, F>(val: T) -> F
where
    T: Intensity + Into<F>,
    F: Copy + Div<Output = F> + From<f32> + PartialOrd,
{
    let val: F = val.into();
    let max: F = T::MAX.into();
    val / max
}

impl<T> Normalize<NormF32> for T
where
    T: Intensity + Into<f32> + Copy,
{
    fn normalize(&self) -> Option<NormF32> {
        let inner = to_float_intensity(*self);
        NormF32::new(inner)
    }

    fn normalize_clamped(&self) -> NormF32 {
        let inner = to_float_intensity(*self);
        NormF32::new_clamped(inner)
    }

    unsafe fn normalize_unchecked(&self) -> NormF32 {
        let inner = to_float_intensity(*self);
        unsafe { NormF32::new_unchecked(inner) }
    }
}

impl<T> Normalize<NormF64> for T
where
    T: Intensity + Into<f64> + Copy,
{
    fn normalize(&self) -> Option<NormF64> {
        let inner = to_float_intensity(*self);
        inner.normalize()
    }

    fn normalize_clamped(&self) -> NormF64 {
        let inner = to_float_intensity(*self);
        inner.normalize_clamped()
    }

    unsafe fn normalize_unchecked(&self) -> NormF64 {
        let inner = to_float_intensity(*self);
        unsafe { inner.normalize_unchecked() }
    }
}

/// A marker trait that represents scalar data types that are normalized.
pub trait Normalized: Intensity {}

/// A normalized [`f32`] value, between 0.0 and 1.0 inclusive.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NormF32(f32);

/// A normalized [`f64`] value, between 0.0 and 1.0 inclusive.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NormF64(f64);

macro_rules! impl_norm {
    ($name:ident, $ty:ty) => {
        impl $name {
            /// Creates a new normalized value.
            ///
            /// If the input value is not between 0.0 and 1.0 inclusive, returns `None`.
            #[must_use]
            pub const fn new(value: $ty) -> Option<Self> {
                if value < 0.0 || value > 1.0 {
                    None
                } else {
                    Some(Self(value))
                }
            }

            /// Creates a new normalized value.
            ///
            /// The input value is clamped to the range [0.0, 1.0].
            #[must_use]
            pub const fn new_clamped(value: $ty) -> Self {
                Self(value.clamp(0.0, 1.0))
            }

            /// Creates a new normalized value without checking the input.
            ///
            /// ## Safety
            ///
            /// If the value is not between 0.0 and 1.0 inclusive, this will lead to undefined behavior.
            #[must_use]
            pub const unsafe fn new_unchecked(value: $ty) -> Self {
                debug_assert!(value >= 0.0 && value <= 1.0);
                Self(value)
            }

            /// Returns the inner value.
            #[must_use]
            pub const fn to_inner(self) -> $ty {
                self.0
            }

            /// Consumes and returns the inner value.
            #[must_use]
            pub const fn into_inner(self) -> $ty {
                self.0
            }
        }

        impl From<$name> for $ty {
            fn from(norm: $name) -> Self {
                norm.0
            }
        }

        impl TryFrom<$ty> for $name {
            type Error = ();

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(())
            }
        }

        impl Intensity for $name {
            const MIN: Self = Self(0.0);
            const MAX: Self = Self(1.0);

            fn from_normalized_f32(value: f32) -> Self {
                Self::new_clamped(value.into())
            }

            #[allow(clippy::cast_possible_truncation)]
            fn from_normalized_f64(value: f64) -> Self {
                Self::new_clamped(value as $ty)
            }
        }

        impl Normalized for $name {}

        impl Normalize<$name> for $ty {
            fn normalize(&self) -> Option<$name> {
                $name::new(*self)
            }

            fn normalize_clamped(&self) -> $name {
                $name::new_clamped(*self)
            }

            unsafe fn normalize_unchecked(&self) -> $name {
                unsafe { $name::new_unchecked(*self) }
            }
        }
    };
}

impl_norm!(NormF32, f32);

impl_norm!(NormF64, f64);

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn norm_f32_new_ok() {
        assert_eq!(NormF32::new(0.5).unwrap().into_inner(), 0.5);
    }

    #[test]
    fn norm_f32_new_too_low() {
        assert!(NormF32::new(-0.1).is_none());
    }

    #[test]
    fn norm_f32_new_too_high() {
        assert!(NormF32::new(1.1).is_none());
    }

    #[test]
    fn norm_f32_new_clamped_ok() {
        assert_eq!(NormF32::new_clamped(0.5).into_inner(), 0.5);
    }

    #[test]
    fn norm_f32_new_clamped_too_low() {
        assert_eq!(NormF32::new_clamped(-0.1).into_inner(), 0.0);
    }

    #[test]
    fn norm_f32_new_clamped_too_high() {
        assert_eq!(NormF32::new_clamped(1.1).into_inner(), 1.0);
    }

    #[test]
    fn norm_f32_new_unchecked() {
        let norm = unsafe { NormF32::new_unchecked(0.5) };
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn try_from_f32_to_norm_f32() {
        let norm: NormF32 = 0.5.try_into().unwrap();
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn from_norm_f32_to_f32() {
        let norm = NormF32::new(0.5).unwrap();
        let value: f32 = norm.into();
        assert_eq!(value, 0.5);
    }

    #[test]
    fn norm_f32_to_inner() {
        let norm = NormF32::new(0.5).unwrap();
        assert_eq!(norm.to_inner(), 0.5);
    }

    #[test]
    fn norm_f32_normalize_is_some() {
        let norm = NormF32::new(0.5).unwrap();
        assert!(norm.normalize().is_some());
    }

    #[test]
    fn norm_f32_normalize_clamped() {
        let norm = NormF32::new(0.5).unwrap();
        assert_eq!(norm.normalize_clamped().into_inner(), 0.5);
    }

    #[test]
    fn norm_f32_normalize_unchecked() {
        let norm = NormF32::new(0.5).unwrap();
        assert_eq!(unsafe { norm.normalize_unchecked() }.into_inner(), 0.5);
    }

    #[test]
    fn f32_normalize_some() {
        let norm: NormF32 = 0.5f32.normalize().unwrap();
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn f32_normalize_none() {
        let norm: Option<NormF32> = (-0.1f32).normalize();
        assert!(norm.is_none());
    }

    #[test]
    fn f32_normalize_clamped() {
        let norm: NormF32 = 1.5f32.normalize_clamped();
        assert_eq!(norm.into_inner(), 1.0);
    }

    #[test]
    fn f32_normalize_unchecked() {
        let norm = unsafe { NormF32::new_unchecked(0.5) };
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn norm_f64_new_ok() {
        assert_eq!(NormF64::new(0.5).unwrap().into_inner(), 0.5);
    }

    #[test]
    fn norm_f64_new_too_low() {
        assert!(NormF64::new(-0.1).is_none());
    }

    #[test]
    fn norm_f64_new_too_high() {
        assert!(NormF64::new(1.1).is_none());
    }

    #[test]
    fn norm_f64_new_clamped_ok() {
        assert_eq!(NormF64::new_clamped(0.5).into_inner(), 0.5);
    }

    #[test]
    fn norm_f64_new_clamped_too_low() {
        assert_eq!(NormF64::new_clamped(-0.1).into_inner(), 0.0);
    }

    #[test]
    fn norm_f64_new_clamped_too_high() {
        assert_eq!(NormF64::new_clamped(1.1).into_inner(), 1.0);
    }

    #[test]
    fn norm_f64_new_unchecked() {
        let norm = unsafe { NormF64::new_unchecked(0.5) };
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn try_from_f64_to_norm_f64() {
        let norm: NormF64 = 0.5f64.try_into().unwrap();
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn from_norm_f64_to_f64() {
        let norm = NormF64::new(0.5).unwrap();
        let value: f64 = norm.into();
        assert_eq!(value, 0.5);
    }

    #[test]
    fn norm_f64_to_inner() {
        let norm = NormF64::new(0.5).unwrap();
        assert_eq!(norm.to_inner(), 0.5);
    }

    #[test]
    fn norm_f64_normalize_is_some() {
        let norm = NormF64::new(0.5).unwrap();
        assert!(norm.normalize().is_some());
    }

    #[test]
    fn norm_f64_normalize_clamped() {
        let norm = NormF64::new(0.5).unwrap();
        assert_eq!(norm.normalize_clamped().into_inner(), 0.5);
    }

    #[test]
    fn norm_f64_normalize_unchecked() {
        let norm = NormF64::new(0.5).unwrap();
        assert_eq!(unsafe { norm.normalize_unchecked() }.into_inner(), 0.5);
    }

    #[test]
    fn f64_normalize_some() {
        let norm: NormF64 = 0.5f64.normalize().unwrap();
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn f64_normalize_none() {
        let norm: Option<NormF64> = (-0.1f64).normalize();
        assert!(norm.is_none());
    }

    #[test]
    fn f64_normalize_clamped() {
        let norm: NormF64 = 1.5f64.normalize_clamped();
        assert_eq!(norm.into_inner(), 1.0);
    }

    #[test]
    fn f64_normalize_unchecked() {
        let norm = unsafe { NormF64::new_unchecked(0.5) };
        assert_eq!(norm.into_inner(), 0.5);
    }

    #[test]
    fn u8_to_norm_f32() {
        let norm: NormF32 = 128u8.normalize().unwrap();
        assert_eq!(norm.into_inner(), 0.501_960_8);
    }

    #[test]
    fn u8_to_norm_f64() {
        let norm: NormF64 = 128u8.normalize().unwrap();
        assert_eq!(norm.into_inner(), 0.501_960_784_313_725_5);
    }
}
