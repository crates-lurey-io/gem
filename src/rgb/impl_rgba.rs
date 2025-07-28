use crate::{
    alpha::{AlphaFirst, AlphaLast, HasAlpha},
    rgb::{HasBlue, HasGreen, HasRed},
    scalar::intensity::Intensity,
};

impl<A, C, T> HasRed<T> for AlphaFirst<A, C>
where
    A: Copy + Intensity,
    C: Copy + HasRed<T>,
{
    fn new_red(value: T) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_red(value);
        color.set_alpha(A::MAX);
        color
    }

    fn red(&self) -> T {
        self.color().red()
    }

    fn set_red(&mut self, value: T) {
        let color = &mut **self;
        color.set_red(value);
    }
}

impl<A, C, T> HasGreen<T> for AlphaFirst<A, C>
where
    A: Copy + Intensity,
    C: Copy + HasGreen<T>,
{
    fn new_green(value: T) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_green(value);
        color.set_alpha(A::MAX);
        color
    }

    fn green(&self) -> T {
        self.color().green()
    }

    fn set_green(&mut self, value: T) {
        let color = &mut **self;
        color.set_green(value);
    }
}

impl<A, C, T> HasBlue<T> for AlphaFirst<A, C>
where
    A: Copy + Intensity,
    C: Copy + HasBlue<T>,
{
    fn new_blue(value: T) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_blue(value);
        color.set_alpha(A::MAX);
        color
    }

    fn blue(&self) -> T {
        self.color().blue()
    }

    fn set_blue(&mut self, value: T) {
        let color = &mut **self;
        color.set_blue(value);
    }
}

impl<A, C, T> HasRed<T> for AlphaLast<A, C>
where
    A: Copy + Intensity,
    C: Copy + HasRed<T>,
{
    fn new_red(value: T) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_red(value);
        color
    }

    fn red(&self) -> T {
        self.color().red()
    }

    fn set_red(&mut self, value: T) {
        let color = &mut **self;
        color.set_red(value);
    }
}

impl<A, C, T> HasGreen<T> for AlphaLast<A, C>
where
    A: Copy + Intensity,
    C: Copy + HasGreen<T>,
{
    fn new_green(value: T) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_green(value);
        color.set_alpha(A::MAX);
        color
    }

    fn green(&self) -> T {
        self.color().green()
    }

    fn set_green(&mut self, value: T) {
        let color = &mut **self;
        color.set_green(value);
    }
}

impl<A, C, T> HasBlue<T> for AlphaLast<A, C>
where
    A: Copy + Intensity,
    C: Copy + HasBlue<T>,
{
    fn new_blue(value: T) -> Self
    where
        Self: Sized + Default,
    {
        let mut color = Self::default();
        color.set_blue(value);
        color.set_alpha(A::MAX);
        color
    }

    fn blue(&self) -> T {
        self.color().blue()
    }

    fn set_blue(&mut self, value: T) {
        let color = &mut **self;
        color.set_blue(value);
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::rgb::{Abgr8888, HasRed};
    use alloc::vec;

    #[test]
    fn abgr8888_new_red() {
        let red = Abgr8888::new_red(0xFF);
        assert_eq!(red, Abgr8888::new(0xFF, 0x00, 0x00, 0xFF));
    }

    #[test]
    fn bytemuck_cast_slice() {
        let colors = vec![
            Abgr8888::new(0xFF, 0x00, 0x00, 0xFF),
            Abgr8888::new(0x00, 0xFF, 0x00, 0xFF),
            Abgr8888::new(0x00, 0x00, 0xFF, 0xFF),
        ];
        let bytes: &[u8] = bytemuck::cast_slice(&colors);
        assert_eq!(bytes.len(), colors.len() * core::mem::size_of::<Abgr8888>());

        #[rustfmt::skip]
        assert_eq!(
            bytes,
            [
                0xFF, 0x00, 0x00, 0xFF, // Red
                0x00, 0xFF, 0x00, 0xFF, // Green
                0x00, 0x00, 0xFF, 0xFF, // Blue
            ]
        );
    }
}
