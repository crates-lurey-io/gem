//! Real-valued interpolation of a single channel, in its native domain.

/// Interpolates a single channel value toward `other` by `t`.
///
/// `t == 0.0` returns `self`; `t == 1.0` returns `other`. Interpolation happens
/// in the channel's own domain (`0..=255` for a `u8`, `0..=31` for a packed
/// 5-bit red, and so on), with no color-space conversion: the channel keeps
/// whatever encoding it already had, which for pixel formats is typically
/// gamma-encoded sRGB.
///
/// Integer channels round to nearest, ties away from zero, and clamp to the
/// channel's representable range. Floating-point channels interpolate exactly
/// and are not clamped.
///
/// This needs neither `std` nor `libm`: integer rounding is `floor(x + 0.5)`.
/// For an all-integer, `const`-callable path over `u8` channels, use
/// [`mix_u8`][crate::channel::mix_u8].
pub trait MixChannel: Copy {
    /// Interpolates `self` toward `other` by `t`.
    #[must_use]
    fn mix_channel(self, other: Self, t: f32) -> Self;
}

impl MixChannel for u8 {
    #[inline]
    #[allow(clippy::suboptimal_flops)]
    fn mix_channel(self, other: Self, t: f32) -> Self {
        // Interpolate in the 0..=255 domain and round once, half away from
        // zero. Staying in this domain (rather than normalizing to [0, 1] and
        // back) makes the result an exact round-to-nearest of a + (b - a) * t,
        // so t = 0.5 yields the true midpoint (e.g. 0, 255 -> 128).
        let a = f32::from(self);
        let b = f32::from(other);
        let v = (a + (b - a) * t).clamp(0.0, 255.0);
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "v is clamped to [0, 255]; +0.5 then truncation rounds half away from zero"
        )]
        {
            (v + 0.5) as Self
        }
    }
}

impl MixChannel for u16 {
    #[inline]
    #[allow(clippy::suboptimal_flops)]
    fn mix_channel(self, other: Self, t: f32) -> Self {
        let a = f32::from(self);
        let b = f32::from(other);
        let v = (a + (b - a) * t).clamp(0.0, 65535.0);
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "v is clamped to [0, 65535]; +0.5 then truncation rounds half away from zero"
        )]
        {
            (v + 0.5) as Self
        }
    }
}

impl MixChannel for f32 {
    #[inline]
    #[allow(clippy::suboptimal_flops)]
    fn mix_channel(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::channel::mix_u8;

    #[test]
    fn u8_endpoints_and_midpoint() {
        assert_eq!(0u8.mix_channel(255, 0.0), 0);
        assert_eq!(0u8.mix_channel(255, 1.0), 255);
        assert_eq!(0u8.mix_channel(255, 0.5), 128); // 127.5 -> 128
        assert_eq!(100u8.mix_channel(101, 0.5), 101); // 100.5 -> 101
    }

    #[test]
    fn u8_midpoint_is_exact_for_all_pairs() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                let got = a.mix_channel(b, 0.5);
                // Round-half-away of the rational midpoint (a + b) / 2.
                let exact = u8::try_from((u16::from(a) + u16::from(b)).div_ceil(2)).unwrap();
                assert_eq!(got, exact, "a={a} b={b}");
            }
        }
    }

    #[test]
    fn u8_clamps_out_of_range_t() {
        assert_eq!(10u8.mix_channel(20, -1.0), 0);
        assert_eq!(250u8.mix_channel(255, 2.0), 255);
    }

    /// The float and integer paths are two implementations of one operation,
    /// which is exactly the situation this crate exists to stop duplicating.
    /// They must not disagree on any input the integer path can express.
    #[test]
    fn u8_agrees_with_the_const_integer_path() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                for t in [0u8, 1, 32, 64, 127, 128, 192, 254, 255] {
                    let float = a.mix_channel(b, f32::from(t) / 255.0);
                    assert_eq!(float, mix_u8(a, b, t), "a={a} b={b} t={t}");
                }
            }
        }
    }

    #[test]
    fn f32_channel_is_exact() {
        assert_eq!(0.0f32.mix_channel(1.0, 0.25), 0.25);
        assert_eq!((-1.0f32).mix_channel(1.0, 0.5), 0.0);
    }

    #[test]
    fn u16_endpoints_and_midpoint() {
        assert_eq!(0u16.mix_channel(65535, 0.0), 0);
        assert_eq!(0u16.mix_channel(65535, 1.0), 65535);
        assert_eq!(0u16.mix_channel(65535, 0.5), 32768); // 32767.5 -> 32768
    }

    #[test]
    fn u16_clamps_out_of_range_t() {
        assert_eq!(10u16.mix_channel(20, -1.0), 0);
        assert_eq!(65_530u16.mix_channel(65_535, 2.0), 65_535);
    }
}
