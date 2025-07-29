macro_rules! impl_with_alpha_packed {
    ($ty:ident, $alpha_shift:expr, $alpha_mask:expr, $alpha_clear:expr) => {
        impl crate::alpha::HasAlpha<u8> for $ty {
            fn alpha(&self) -> u8 {
                ((self.packed >> $alpha_shift) & $alpha_mask) as u8
            }

            fn set_alpha(&mut self, value: u8) {
                self.packed = (self.packed & $alpha_clear)
                    | ((u16::from(value) & $alpha_mask) << $alpha_shift);
            }
        }
    };
}

pub(crate) use impl_with_alpha_packed;

#[cfg(test)]
mod tests {
    use crate::{alpha::HasAlpha, rgb::Argb1555};

    #[test]
    fn set_alpha() {
        let mut color = Argb1555::from_rgb(0, 0, 0);
        color.set_alpha(1);
        assert_eq!(color.alpha(), 1);
    }
}
