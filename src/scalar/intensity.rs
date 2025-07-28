/// A trait that represents scalar data types used as an intensity value.
pub trait Intensity {
    /// The minimum value of this intensity type.
    const MIN: Self;

    /// The maximum value of this intensity type.
    const MAX: Self;
}

impl Intensity for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl Intensity for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn u8_min_max() {
        assert_eq!(<u8 as Intensity>::MIN, 0);
        assert_eq!(<u8 as Intensity>::MAX, 255);
    }

    #[test]
    fn u16_min_max() {
        assert_eq!(<u16 as Intensity>::MIN, 0);
        assert_eq!(<u16 as Intensity>::MAX, 65535);
    }
}
