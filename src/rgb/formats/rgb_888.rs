use crate::rgb::Rgb;

/// 8-bit RGB color representation.
///
/// Each component is represented by 8 bits, stored contiguously with no padding.
///
/// ## Layout
///
/// ```c
/// struct Rgb888 {
///   uint8_t r;
///   uint8_t g;
///   uint8_t b;
/// }
/// ```
///
/// `size_of::<Rgb888>() == 3` and `align_of::<Rgb888>() == 1` — there is no
/// padding, since `Rgb<u8>` is `#[repr(C)]` over three `u8` fields (alignment
/// 1). This differs from many GPU/graphics APIs that pad 24-bit-per-pixel
/// formats to 32 bits for alignment; if you need that padding, add it
/// explicitly at the buffer level rather than assuming this type provides it.
///
/// ## Examples
///
/// To create an `Rgb888` color from a packed representation:
///
/// ```rust
/// use gem::rgb::Rgb888;
///
/// let color = Rgb888::new(0xFF0000);
/// ```
///
/// To create an `Rgb888` color from individual components:
///
/// ```rust
/// use gem::rgb::Rgb888;
///
/// let color = Rgb888::from_rgb(255, 0, 0);
/// ```
pub type Rgb888 = Rgb<u8>;

impl Rgb888 {
    /// Creates a new RGB color from the top 24-bits of a packed ([`u32`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 23-16 | 15-8  | 7-0  |
    /// |   R   |   G   |   B  |
    /// ```
    ///
    /// Any additional bits in the packed representation are ignored.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// assert_eq!(Rgb888::new(0xFF0000), Rgb888::from_rgb(255, 0, 0));
    /// ```
    #[must_use]
    pub const fn new(packed: u32) -> Self {
        let r = ((packed >> 16) & 0xFF) as u8;
        let g = ((packed >> 8) & 0xFF) as u8;
        let b = (packed & 0xFF) as u8;
        Self::from_rgb(r, g, b)
    }

    /// Multiplies each channel by the matching channel of `factor`.
    ///
    /// The W3C `multiply` blend mode, per channel, via
    /// [`multiply_u8`][crate::channel::multiply_u8]. White is the exact
    /// identity, so a color can be multiplied by white any number of times
    /// without drifting darker.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// let grass = Rgb888::from_rgb(200, 180, 60);
    /// let shadowed = grass.multiply(Rgb888::from_rgb(128, 128, 128));
    /// assert_eq!(shadowed, Rgb888::from_rgb(100, 90, 30));
    ///
    /// assert_eq!(grass.multiply(Rgb888::from_rgb(255, 255, 255)), grass);
    /// ```
    #[must_use]
    pub const fn multiply(self, factor: Self) -> Self {
        use crate::channel::multiply_u8;
        let (r, g, b) = self.to_rgb();
        let (fr, fg, fb) = factor.to_rgb();
        Self::from_rgb(multiply_u8(r, fr), multiply_u8(g, fg), multiply_u8(b, fb))
    }

    /// Screens each channel against the matching channel of `other`.
    ///
    /// The exact complement of [`multiply`][Self::multiply], via
    /// [`screen_u8`][crate::channel::screen_u8]: where multiply can only darken,
    /// screen can only lighten, and black is the identity.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// let grass = Rgb888::from_rgb(200, 180, 60);
    /// assert_eq!(grass.screen(Rgb888::from_rgb(0, 0, 0)), grass);
    /// assert_eq!(
    ///     grass.screen(Rgb888::from_rgb(255, 255, 255)),
    ///     Rgb888::from_rgb(255, 255, 255),
    /// );
    /// ```
    #[must_use]
    pub const fn screen(self, other: Self) -> Self {
        use crate::channel::screen_u8;
        let (r, g, b) = self.to_rgb();
        let (or, og, ob) = other.to_rgb();
        Self::from_rgb(screen_u8(r, or), screen_u8(g, og), screen_u8(b, ob))
    }

    /// Interpolates each channel toward `other` by `t / 255`.
    ///
    /// The `const`, all-integer counterpart of [`Mix::mix`][crate::Mix::mix],
    /// via [`mix_u8`][crate::channel::mix_u8]. It carries the `_u8` suffix
    /// precisely because it is *not* interchangeable with `mix`: `t` here is a
    /// fraction of 255, not a real number in `[0.0, 1.0]`. The two agree on
    /// every `t` this one can express.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// let grass = Rgb888::from_rgb(200, 180, 60);
    /// let white = Rgb888::from_rgb(255, 255, 255);
    ///
    /// // Endpoints are exact.
    /// assert_eq!(grass.mix_u8(white, 0), grass);
    /// assert_eq!(grass.mix_u8(white, 255), white);
    ///
    /// // A damage flash, most of the way to white.
    /// assert_eq!(grass.mix_u8(white, 192), Rgb888::from_rgb(241, 236, 207));
    /// ```
    #[must_use]
    pub const fn mix_u8(self, other: Self, t: u8) -> Self {
        use crate::channel::mix_u8;
        let (r, g, b) = self.to_rgb();
        let (or, og, ob) = other.to_rgb();
        Self::from_rgb(mix_u8(r, or, t), mix_u8(g, og, t), mix_u8(b, ob, t))
    }

    /// Squared euclidean distance to `other`, via [`distance_sq`][crate::rgb::distance_sq].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Rgb888;
    ///
    /// let black = Rgb888::from_rgb(0, 0, 0);
    /// assert_eq!(black.distance_sq(black), 0);
    /// assert_eq!(black.distance_sq(Rgb888::from_rgb(255, 255, 255)), 195_075);
    /// ```
    #[must_use]
    pub const fn distance_sq(self, other: Self) -> u32 {
        crate::rgb::distance_sq(self.to_rgb(), other.to_rgb())
    }
}

impl From<[u8; 3]> for Rgb888 {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl From<Rgb888> for [u8; 3] {
    fn from(color: Rgb888) -> Self {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        [color.red(), color.green(), color.blue()]
    }
}

impl core::fmt::LowerHex for Rgb888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        let packed =
            (u32::from(self.red()) << 16) | (u32::from(self.green()) << 8) | u32::from(self.blue());
        write!(f, "{packed:06x}")
    }
}

impl core::fmt::UpperHex for Rgb888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        let packed =
            (u32::from(self.red()) << 16) | (u32::from(self.green()) << 8) | u32::from(self.blue());
        write!(f, "{packed:06X}")
    }
}

impl core::fmt::Display for Rgb888 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{self:x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb888_new() {
        assert_eq!(Rgb888::new(0x00FF_0000), Rgb888::from_rgb(255, 0, 0));
        assert_eq!(Rgb888::new(0x0000_FF00), Rgb888::from_rgb(0, 255, 0));
        assert_eq!(Rgb888::new(0x0000_00FF), Rgb888::from_rgb(0, 0, 255));
    }

    #[test]
    fn from_array() {
        assert_eq!(Rgb888::from([255_u8, 0, 0]), Rgb888::from_rgb(255, 0, 0));
    }

    #[test]
    fn to_rgb_is_const() {
        const RGB: (u8, u8, u8) = Rgb888::from_rgb(1, 2, 3).to_rgb();
        assert_eq!(RGB, (1, 2, 3));
    }

    #[test]
    fn const_ops_match_their_channel_primitives() {
        use crate::channel::{mix_u8, multiply_u8, screen_u8};

        let a = Rgb888::from_rgb(200, 180, 60);
        let b = Rgb888::from_rgb(50, 90, 210);

        assert_eq!(
            a.multiply(b),
            Rgb888::from_rgb(
                multiply_u8(200, 50),
                multiply_u8(180, 90),
                multiply_u8(60, 210)
            )
        );
        assert_eq!(
            a.screen(b),
            Rgb888::from_rgb(screen_u8(200, 50), screen_u8(180, 90), screen_u8(60, 210))
        );
        assert_eq!(
            a.mix_u8(b, 64),
            Rgb888::from_rgb(
                mix_u8(200, 50, 64),
                mix_u8(180, 90, 64),
                mix_u8(60, 210, 64)
            )
        );
    }

    /// The `_u8` suffix exists because the two mixes take different `t`
    /// domains, not because they compute different things.
    #[test]
    fn mix_u8_agrees_with_the_mix_trait() {
        use crate::Mix as _;

        let a = Rgb888::from_rgb(200, 180, 60);
        let b = Rgb888::from_rgb(50, 90, 210);
        for t in 0..=255u8 {
            assert_eq!(a.mix_u8(b, t), a.mix(b, f32::from(t) / 255.0), "t={t}");
        }
    }

    #[test]
    fn const_ops_are_usable_in_const_contexts() {
        const SHADOWED: Rgb888 = Rgb888::from_rgb(200, 180, 60).multiply(Rgb888::new(0x0080_8080));
        const FAR: u32 = Rgb888::from_rgb(0, 0, 0).distance_sq(Rgb888::from_rgb(255, 255, 255));
        assert_eq!(SHADOWED, Rgb888::from_rgb(100, 90, 30));
        assert_eq!(FAR, 195_075);
    }

    #[test]
    fn multiply_and_screen_are_complements() {
        let a = Rgb888::from_rgb(200, 180, 60);
        let b = Rgb888::from_rgb(50, 90, 210);
        let invert = |c: Rgb888| {
            let (r, g, b) = c.to_rgb();
            Rgb888::from_rgb(255 - r, 255 - g, 255 - b)
        };
        assert_eq!(a.screen(b), invert(invert(a).multiply(invert(b))));
    }

    #[test]
    #[cfg(feature = "std")]
    fn into_array() {
        let arr: [u8; 3] = Rgb888::from_rgb(255, 128, 0).into();
        assert_eq!(arr, [255, 128, 0]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn lower_hex() {
        assert_eq!(format!("{:x}", Rgb888::from_rgb(255, 128, 0)), "ff8000");
    }

    #[test]
    #[cfg(feature = "std")]
    fn upper_hex() {
        assert_eq!(format!("{:X}", Rgb888::from_rgb(255, 128, 0)), "FF8000");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_roundtrip() {
        let c = Rgb888::from_rgb(1, 2, 3);
        let json = serde_json::to_string(&c).unwrap();
        let back: Rgb888 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, c);
    }

    #[cfg(feature = "std")]
    #[test]
    fn display() {
        assert_eq!(Rgb888::from_rgb(255, 128, 0).to_string(), "#ff8000");
    }
}
