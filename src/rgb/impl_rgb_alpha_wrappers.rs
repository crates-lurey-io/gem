use crate::{
    alpha::{AlphaFirst, AlphaLast},
    rgb::{HasBlue, HasGreen, HasRed},
};

impl<A, C> HasRed for AlphaFirst<A, C>
where
    A: Copy,
    C: Copy + HasRed,
{
    type Component = C::Component;

    fn red(&self) -> Self::Component {
        self.color().red()
    }

    fn set_red(&mut self, value: Self::Component) {
        *self = AlphaFirst::with_color(self.alpha(), self.color().with_red(value));
    }
}

impl<A, C> HasGreen for AlphaFirst<A, C>
where
    A: Copy,
    C: Copy + HasGreen,
{
    type Component = C::Component;

    fn green(&self) -> Self::Component {
        self.color().green()
    }

    fn set_green(&mut self, value: Self::Component) {
        *self = AlphaFirst::with_color(self.alpha(), self.color().with_green(value));
    }
}

impl<A, C> HasBlue for AlphaFirst<A, C>
where
    A: Copy,
    C: Copy + HasBlue,
{
    type Component = C::Component;

    fn blue(&self) -> Self::Component {
        self.color().blue()
    }

    fn set_blue(&mut self, value: Self::Component) {
        *self = AlphaFirst::with_color(self.alpha(), self.color().with_blue(value));
    }
}

impl<A, C> HasRed for AlphaLast<A, C>
where
    A: Copy,
    C: Copy + HasRed,
{
    type Component = C::Component;

    fn red(&self) -> Self::Component {
        self.color().red()
    }

    fn set_red(&mut self, value: Self::Component) {
        *self = AlphaLast::with_color(self.alpha(), self.color().with_red(value));
    }
}

impl<A, C> HasGreen for AlphaLast<A, C>
where
    A: Copy,
    C: Copy + HasGreen,
{
    type Component = C::Component;

    fn green(&self) -> Self::Component {
        self.color().green()
    }

    fn set_green(&mut self, value: Self::Component) {
        *self = AlphaLast::with_color(self.alpha(), self.color().with_green(value));
    }
}

impl<A, C> HasBlue for AlphaLast<A, C>
where
    A: Copy,
    C: Copy + HasBlue,
{
    type Component = C::Component;

    fn blue(&self) -> Self::Component {
        self.color().blue()
    }

    fn set_blue(&mut self, value: Self::Component) {
        *self = AlphaLast::with_color(self.alpha(), self.color().with_blue(value));
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
