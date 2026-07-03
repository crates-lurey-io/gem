mod define_packed;
pub(crate) use define_packed::{define_packed_argb, define_packed_rgb, impl_packed_u16_boilerplate};

mod impl_rgb_packed;
pub(crate) use impl_rgb_packed::impl_rgb_packed;

mod impl_rgb_with_fields;
pub(crate) use impl_rgb_with_fields::impl_rgb_with_fields;

mod impl_with_alpha_packed;
pub(crate) use impl_with_alpha_packed::impl_with_alpha_packed;
