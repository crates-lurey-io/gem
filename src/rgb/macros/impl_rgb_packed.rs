macro_rules! impl_rgb_packed {
    (
        $ty:ident,
        red:  { shift: $rshift:expr, mask: $rmask:expr, clear: $rclear:expr },
        green:{ shift: $gshift:expr, mask: $gmask:expr, clear: $gclear:expr },
        blue: { shift: $bshift:expr, mask: $bmask:expr, clear: $bclear:expr }
    ) => {
        impl crate::rgb::HasRed<u8> for $ty {
            fn red(&self) -> u8 {
                ((self.packed >> $rshift) & $rmask) as u8
            }
            fn set_red(&mut self, value: u8) {
                self.packed = (self.packed & $rclear) | ((u16::from(value) & $rmask) << $rshift);
            }
        }

        impl crate::rgb::HasGreen<u8> for $ty {
            fn green(&self) -> u8 {
                ((self.packed >> $gshift) & $gmask) as u8
            }
            fn set_green(&mut self, value: u8) {
                self.packed = (self.packed & $gclear) | ((u16::from(value) & $gmask) << $gshift);
            }
        }

        impl crate::rgb::HasBlue<u8> for $ty {
            fn blue(&self) -> u8 {
                ((self.packed >> $bshift) & $bmask) as u8
            }
            fn set_blue(&mut self, value: u8) {
                self.packed = (self.packed & $bclear) | ((u16::from(value) & $bmask) << $bshift);
            }
        }
    };
}

pub(crate) use impl_rgb_packed;

#[cfg(test)]
mod tests {
    use crate::rgb::{HasBlue, HasGreen, HasRed, Rgb565};

    #[test]
    fn set_red() {
        let mut color = Rgb565::from_rgb(0, 0, 0);
        color.set_red(15);
        assert_eq!(color.red(), 15);
    }

    #[test]
    fn set_green() {
        let mut color = Rgb565::from_rgb(0, 0, 0);
        color.set_green(15);
        assert_eq!(color.green(), 15);
    }

    #[test]
    fn set_blue() {
        let mut color = Rgb565::from_rgb(0, 0, 0);
        color.set_blue(15);
        assert_eq!(color.blue(), 15);
    }
}
