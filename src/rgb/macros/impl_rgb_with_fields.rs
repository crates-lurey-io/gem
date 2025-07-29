macro_rules! impl_rgb_with_fields {
    ($ty:ident<$t:ident>) => {
        impl<$t> crate::rgb::HasRed<$t> for $ty<$t>
        where
            $t: Copy,
        {
            fn red(&self) -> $t {
                self.r
            }

            fn set_red(&mut self, value: $t) {
                self.r = value;
            }
        }

        impl<$t> crate::rgb::HasGreen<$t> for $ty<$t>
        where
            $t: Copy,
        {
            fn green(&self) -> $t {
                self.g
            }

            fn set_green(&mut self, value: $t) {
                self.g = value;
            }
        }

        impl<$t> crate::rgb::HasBlue<$t> for $ty<$t>
        where
            $t: Copy,
        {
            fn blue(&self) -> $t {
                self.b
            }

            fn set_blue(&mut self, value: $t) {
                self.b = value;
            }
        }
    };
}

pub(crate) use impl_rgb_with_fields;
