use crate::{
    alpha::{AlphaFirst, AlphaLast},
    rgb::{HasBlue, HasGreen, HasRed},
};

impl<A, C, T> HasRed<T> for AlphaFirst<A, C>
where
    C: Copy + HasRed<T>,
{
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
    C: Copy + HasGreen<T>,
{
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
    C: Copy + HasBlue<T>,
{
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
    C: Copy + HasRed<T>,
{
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
    C: Copy + HasGreen<T>,
{
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
    C: Copy + HasBlue<T>,
{
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
    use crate::rgb::Rgb888;

    use super::*;

    #[test]
    fn alpha_first_set_red() {
        let mut color = AlphaFirst::<u8, Rgb888>::with_color(255, Rgb888::from_rgb(0, 0, 0));
        color.set_red(128);
        assert_eq!(color.red(), 128);
    }

    #[test]
    fn alpha_first_set_green() {
        let mut color = AlphaFirst::<u8, Rgb888>::with_color(255, Rgb888::from_rgb(0, 0, 0));
        color.set_green(128);
        assert_eq!(color.green(), 128);
    }

    #[test]
    fn alpha_first_set_blue() {
        let mut color = AlphaFirst::<u8, Rgb888>::with_color(255, Rgb888::from_rgb(0, 0, 0));
        color.set_blue(128);
        assert_eq!(color.blue(), 128);
    }

    #[test]
    fn alpha_last_set_red() {
        let mut color = AlphaLast::<u8, Rgb888>::with_color(255, Rgb888::from_rgb(0, 0, 0));
        color.set_red(128);
        assert_eq!(color.red(), 128);
    }

    #[test]
    fn alpha_last_set_green() {
        let mut color = AlphaLast::<u8, Rgb888>::with_color(255, Rgb888::from_rgb(0, 0, 0));
        color.set_green(128);
        assert_eq!(color.green(), 128);
    }

    #[test]
    fn alpha_last_set_blue() {
        let mut color = AlphaLast::<u8, Rgb888>::with_color(255, Rgb888::from_rgb(0, 0, 0));
        color.set_blue(128);
        assert_eq!(color.blue(), 128);
    }
}
