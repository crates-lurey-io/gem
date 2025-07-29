#[cfg(not(feature = "libm"))]
compile_error!("This module requires the `libm` feature to be enabled.");

extern crate libm;

#[inline]
pub fn round_f32(value: f32) -> f32 {
    libm::roundf(value)
}

#[inline]
pub fn round_f64(value: f64) -> f64 {
    libm::round(value)
}
