#[cfg(not(feature = "std"))]
compile_error!("This module requires the `std` feature to be enabled.");

extern crate std;

#[inline]
pub fn round_f32(value: f32) -> f32 {
    value.round()
}

#[inline]
pub fn round_f64(value: f64) -> f64 {
    value.round()
}
