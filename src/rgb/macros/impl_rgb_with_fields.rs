macro_rules! impl_rgb_with_fields {
    ($ty:ident<$t:ident>) => {
        impl<$t> crate::rgb::HasRed for $ty<$t>
        where
            $t: Copy,
        {
            type Component = $t;

            fn red(&self) -> Self::Component {
                self.r
            }

            fn set_red(&mut self, value: Self::Component) {
                self.r = value;
            }
        }

        impl<$t> crate::rgb::HasGreen for $ty<$t>
        where
            $t: Copy,
        {
            type Component = $t;

            fn green(&self) -> Self::Component {
                self.g
            }

            fn set_green(&mut self, value: Self::Component) {
                self.g = value;
            }
        }

        impl<$t> crate::rgb::HasBlue for $ty<$t>
        where
            $t: Copy,
        {
            type Component = $t;

            fn blue(&self) -> Self::Component {
                self.b
            }

            fn set_blue(&mut self, value: Self::Component) {
                self.b = value;
            }
        }
    };
}

pub(crate) use impl_rgb_with_fields;
