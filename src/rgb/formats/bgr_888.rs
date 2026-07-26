use crate::rgb::Bgr;

/// 8-bit BGR color representation.
///
/// Each component is represented by 8 bits, with the order being blue, green, and red,
/// stored contiguously with no padding.
///
/// ## Layout
///
/// ```c
/// struct Bgr888 {
///   uint8_t b;
///   uint8_t g;
///   uint8_t r;
/// }
/// ```
///
/// `size_of::<Bgr888>() == 3` and `align_of::<Bgr888>() == 1` — there is no
/// padding (see [`Rgb888`][crate::rgb::Rgb888] for the same note in more detail).
///
/// ## Examples
///
/// To create a `Bgr888` color from a packed representation:
///
/// ```rust
/// use gem::rgb::Bgr888;
///
/// let color = Bgr888::new(0x0000FF);
/// ```
///
/// To create a `Bgr888` color from individual components:
///
/// ```rust
/// use gem::rgb::Bgr888;
///
/// let color = Bgr888::from_bgr(255, 0, 0);
/// ```
pub type Bgr888 = Bgr<u8>;

impl Bgr888 {
    /// Creates a new BGR color from the top 24-bits of a packed ([`u32`]) representation.
    ///
    /// The packed representation is expected to have the format:
    ///
    /// ```txt
    /// | 23-16 | 15-8  | 7-0  |
    /// |   B   |   G   |   R  |
    /// ```
    ///
    /// Any additional bits in the packed representation are ignored.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// assert_eq!(Bgr888::new(0x0000FF), Bgr888::from_bgr(0, 0, 255));
    /// ```
    #[must_use]
    pub const fn new(packed: u32) -> Self {
        let b = ((packed >> 16) & 0xFF) as u8;
        let g = ((packed >> 8) & 0xFF) as u8;
        let r = (packed & 0xFF) as u8;
        Self::from_bgr(b, g, r)
    }

    /// Multiplies each channel by the matching channel of `factor`.
    ///
    /// Matched by color, not by position: see [`Rgb888::multiply`] for the
    /// operation's semantics.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// let grass = Bgr888::from_bgr(60, 180, 200);
    /// let shadowed = grass.multiply(Bgr888::from_bgr(128, 128, 128));
    /// assert_eq!(shadowed, Bgr888::from_bgr(30, 90, 100));
    /// ```
    ///
    /// [`Rgb888::multiply`]: crate::rgb::Rgb888::multiply
    #[must_use]
    pub const fn multiply(self, factor: Self) -> Self {
        use crate::channel::multiply_u8;
        let (b, g, r) = self.to_bgr();
        let (fb, fg, fr) = factor.to_bgr();
        Self::from_bgr(multiply_u8(b, fb), multiply_u8(g, fg), multiply_u8(r, fr))
    }

    /// Screens each channel against the matching channel of `other`.
    ///
    /// See [`Rgb888::screen`][crate::rgb::Rgb888::screen].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// let grass = Bgr888::from_bgr(60, 180, 200);
    /// assert_eq!(grass.screen(Bgr888::from_bgr(0, 0, 0)), grass);
    /// ```
    #[must_use]
    pub const fn screen(self, other: Self) -> Self {
        use crate::channel::screen_u8;
        let (b, g, r) = self.to_bgr();
        let (ob, og, or) = other.to_bgr();
        Self::from_bgr(screen_u8(b, ob), screen_u8(g, og), screen_u8(r, or))
    }

    /// Interpolates each channel toward `other` by `t / 255`.
    ///
    /// See [`Rgb888::mix_u8`][crate::rgb::Rgb888::mix_u8], including why this
    /// carries a `_u8` suffix and [`Mix::mix`][crate::Mix::mix] does not.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// let grass = Bgr888::from_bgr(60, 180, 200);
    /// let white = Bgr888::from_bgr(255, 255, 255);
    /// assert_eq!(grass.mix_u8(white, 0), grass);
    /// assert_eq!(grass.mix_u8(white, 255), white);
    /// ```
    #[must_use]
    pub const fn mix_u8(self, other: Self, t: u8) -> Self {
        use crate::channel::mix_u8;
        let (b, g, r) = self.to_bgr();
        let (ob, og, or) = other.to_bgr();
        Self::from_bgr(mix_u8(b, ob, t), mix_u8(g, og, t), mix_u8(r, or, t))
    }

    /// Squared euclidean distance to `other`, via [`distance_sq`][crate::rgb::distance_sq].
    ///
    /// Storage order does not affect the result, so this agrees with
    /// [`Rgb888::distance_sq`][crate::rgb::Rgb888::distance_sq] for the same
    /// two colors.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use gem::rgb::Bgr888;
    ///
    /// let black = Bgr888::from_bgr(0, 0, 0);
    /// assert_eq!(black.distance_sq(Bgr888::from_bgr(255, 255, 255)), 195_075);
    /// ```
    #[must_use]
    pub const fn distance_sq(self, other: Self) -> u32 {
        crate::rgb::distance_sq(self.to_rgb(), other.to_rgb())
    }
}

impl From<[u8; 3]> for Bgr888 {
    /// Creates a `Bgr888` from `[b, g, r]` — bytes in BGR (memory) order.
    fn from([b, g, r]: [u8; 3]) -> Self {
        Self::from_bgr(b, g, r)
    }
}

impl From<Bgr888> for [u8; 3] {
    /// Returns `[b, g, r]` — bytes in BGR (memory) order.
    fn from(color: Bgr888) -> Self {
        use crate::rgb::{HasBlue, HasGreen, HasRed};
        [color.blue(), color.green(), color.red()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bgr888_new() {
        assert_eq!(Bgr888::new(0x0000_00FF), Bgr888::from_bgr(0, 0, 255));
        assert_eq!(Bgr888::new(0x0000_FF00), Bgr888::from_bgr(0, 255, 0));
        assert_eq!(Bgr888::new(0x00FF_0000), Bgr888::from_bgr(255, 0, 0));
    }

    #[test]
    fn to_bgr_is_memory_order_and_to_rgb_is_color_order() {
        const BGR: (u8, u8, u8) = Bgr888::from_bgr(1, 2, 3).to_bgr();
        const RGB: (u8, u8, u8) = Bgr888::from_bgr(1, 2, 3).to_rgb();
        assert_eq!(BGR, (1, 2, 3));
        assert_eq!(RGB, (3, 2, 1));
    }

    /// Storage order must not leak into the arithmetic. A `Bgr888` and an
    /// `Rgb888` holding the same color have to produce the same result, which
    /// is the whole reason these take a color type rather than a bare tuple.
    #[test]
    fn ops_track_color_not_storage_order() {
        use crate::rgb::Rgb888;

        let a_bgr = Bgr888::from_bgr(60, 180, 200);
        let b_bgr = Bgr888::from_bgr(210, 90, 50);
        let a_rgb = Rgb888::from_rgb(200, 180, 60);
        let b_rgb = Rgb888::from_rgb(50, 90, 210);

        assert_eq!(
            a_bgr.multiply(b_bgr).to_rgb(),
            a_rgb.multiply(b_rgb).to_rgb()
        );
        assert_eq!(a_bgr.screen(b_bgr).to_rgb(), a_rgb.screen(b_rgb).to_rgb());
        assert_eq!(
            a_bgr.mix_u8(b_bgr, 64).to_rgb(),
            a_rgb.mix_u8(b_rgb, 64).to_rgb()
        );
        assert_eq!(a_bgr.distance_sq(b_bgr), a_rgb.distance_sq(b_rgb));
    }
}
