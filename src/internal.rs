pub mod math;

/// Prevents external implementations of traits that are not stable for public use.
pub trait Sealed {}

impl Sealed for u8 {}
impl Sealed for u16 {}
impl Sealed for f32 {}
impl Sealed for f64 {}
