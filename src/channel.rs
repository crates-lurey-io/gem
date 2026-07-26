//! Integer channel arithmetic, in each channel's own native domain.
//!
//! An 8-bit channel is a fixed-point number: `0` is `0.0`, `255` is `1.0`, and
//! every step is `1/255`. The operations here work directly in that domain
//! rather than normalizing to `f32` and back, which is what makes them
//! `const fn` and available in the strictest `no_std` build with no math
//! backend at all.
//!
//! | Operation | Meaning | Identity |
//! |-----------|---------|----------|
//! | [`multiply_u8`] | `a * b`, the W3C `multiply` blend mode; also premultiply-by-alpha | `b == 255` |
//! | [`screen_u8`] | `1 - (1 - a) * (1 - b)`, the exact complement of multiply | `b == 0` |
//! | [`mix_u8`] | `a + (b - a) * t`, linear interpolation | `t == 0` |
//!
//! For interpolation with a real-valued factor, or across formats whose
//! channels are not `u8`, see [`MixChannel`] and [`crate::Mix`].
//!
//! # Rounding
//!
//! Every function in this module returns the *round-to-nearest* result, with
//! ties away from zero. This is a crate-wide invariant, not a per-function
//! detail — see the [crate-level docs][crate#rounding].
//!
//! The alternative, truncation, is the cheaper `(v + (v >> 8) + 1) >> 8`
//! shift trick, which is exactly `floor(v / 255)` for every reachable input.
//! It biases every result downward by up to a full step, so a pixel that is
//! composited repeatedly drifts darker on each pass. None of these functions
//! do that.
//!
//! # Examples
//!
//! ```rust
//! use gem::channel::{mix_u8, multiply_u8, screen_u8};
//!
//! // Multiply can only darken; white is its identity.
//! assert_eq!(multiply_u8(200, 255), 200);
//! assert_eq!(multiply_u8(200, 128), 100);
//!
//! // Screen can only lighten; black is its identity.
//! assert_eq!(screen_u8(200, 0), 200);
//!
//! // Mix reaches both endpoints exactly.
//! assert_eq!(mix_u8(0, 255, 0), 0);
//! assert_eq!(mix_u8(0, 255, 255), 255);
//! ```

mod mix;
pub use mix::MixChannel;

/// Multiplies two 8-bit channels, treating each as a fraction of `255`.
///
/// This is `a * b / 255` rounded to nearest, which is simultaneously the W3C
/// `multiply` separable blend mode, premultiplication of a color channel by an
/// alpha channel, and "scale `a` by the factor `b`".
///
/// `255` is the identity and `0` is the annihilator, both exactly: no amount of
/// repeated multiplication by white will drift a channel darker, and no
/// non-zero input survives multiplication by black.
///
/// ## Examples
///
/// ```rust
/// use gem::channel::multiply_u8;
///
/// assert_eq!(multiply_u8(255, 255), 255);
/// assert_eq!(multiply_u8(200, 0), 0);
/// assert_eq!(multiply_u8(200, 128), 100);
///
/// // Usable in a `const` context.
/// const HALF_GREY: u8 = multiply_u8(128, 128);
/// assert_eq!(HALF_GREY, 64);
/// ```
#[must_use]
pub const fn multiply_u8(a: u8, b: u8) -> u8 {
    // `a * b` peaks at 65025, and `+ 127` biases the truncating divide to round
    // to nearest: `v / 255` can never land exactly on a half step for integer
    // `v`, so there is no tie to break and this is exact for all 65536 inputs.
    // The quotient is at most 255, so the cast cannot truncate.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "(65025 + 127) / 255 == 255, which fits in u8"
    )]
    {
        ((a as u16 * b as u16 + 127) / 255) as u8
    }
}

/// Screens two 8-bit channels: `1 - (1 - a) * (1 - b)`, in the `0..=255` domain.
///
/// The exact complement of [`multiply_u8`]: where multiply can only darken,
/// screen can only lighten, and `screen(a, b) == 255 - multiply(255 - a, 255 - b)`
/// holds for every input pair.
///
/// `0` is the identity and `255` is the annihilator.
///
/// ## Examples
///
/// ```rust
/// use gem::channel::{multiply_u8, screen_u8};
///
/// assert_eq!(screen_u8(200, 0), 200);
/// assert_eq!(screen_u8(200, 255), 255);
///
/// // Complement of multiply, by construction.
/// assert_eq!(screen_u8(50, 90), 255 - multiply_u8(255 - 50, 255 - 90));
/// ```
#[must_use]
pub const fn screen_u8(a: u8, b: u8) -> u8 {
    255 - multiply_u8(255 - a, 255 - b)
}

/// Interpolates from `a` toward `b` by `t / 255`.
///
/// `t == 0` returns `a` and `t == 255` returns `b`, both exactly. Intermediate
/// values round to nearest, symmetrically: mixing `a` toward a brighter `b`
/// and mixing `b` toward a darker `a` by the same `t` move the same distance,
/// so an animated pulse does not drift in either direction.
///
/// This is the integer-domain counterpart of [`MixChannel::mix_channel`], which
/// takes a real-valued `t` instead. The two agree wherever both are exact; this
/// one is `const` and needs no floating point.
///
/// ## Examples
///
/// ```rust
/// use gem::channel::mix_u8;
///
/// // Endpoints are exact.
/// assert_eq!(mix_u8(10, 200, 0), 10);
/// assert_eq!(mix_u8(10, 200, 255), 200);
///
/// // Rounding is symmetric in both directions.
/// assert_eq!(mix_u8(100, 200, 64) - 100, 200 - mix_u8(200, 100, 64));
///
/// // Usable in a `const` context.
/// const FLASH: u8 = mix_u8(0, 254, 128);
/// assert_eq!(FLASH, 127);
/// ```
#[must_use]
pub const fn mix_u8(a: u8, b: u8, t: u8) -> u8 {
    let (a, b, t) = (a as i32, b as i32, t as i32);

    // Written as `a + (b - a) * t` rather than `a * (255 - t) + b * t` so both
    // endpoints come out exact with no rounding applied at all.
    let delta = (b - a) * t;

    // Round half away from zero, so a mix toward a darker color and a mix
    // toward a lighter one lose the same amount to rounding.
    let rounded = if delta >= 0 {
        (delta + 127) / 255
    } else {
        (delta - 127) / 255
    };

    // `a` is in 0..=255 and `a + rounded` always lands between `a` and `b`.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "the result is bounded by a and b, both of which are u8"
    )]
    {
        (a + rounded) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-to-nearest (ties away from zero) of `n / 255`, as a reference.
    fn round_div_255(n: i32) -> i32 {
        if n >= 0 {
            (2 * n + 255) / 510
        } else {
            -((-2 * n + 255) / 510)
        }
    }

    #[test]
    fn multiply_is_exact_round_to_nearest_for_every_input() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                let expected = round_div_255(i32::from(a) * i32::from(b));
                assert_eq!(i32::from(multiply_u8(a, b)), expected, "a={a} b={b}");
            }
        }
    }

    #[test]
    fn multiply_endpoints_are_exact() {
        for c in 0..=255u8 {
            assert_eq!(multiply_u8(c, 255), c, "white is not the identity for {c}");
            assert_eq!(multiply_u8(255, c), c, "white is not the identity for {c}");
            assert_eq!(multiply_u8(c, 0), 0, "black did not annihilate {c}");
        }
    }

    #[test]
    fn multiply_is_commutative_and_monotonic() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                assert_eq!(multiply_u8(a, b), multiply_u8(b, a), "a={a} b={b}");
                assert!(multiply_u8(a, b) <= a, "multiply brightened a={a} b={b}");
            }
        }
    }

    /// The truncating shift trick some compositors use, for contrast.
    fn floor_div_255(v: u32) -> u32 {
        (v + (v >> 8) + 1) >> 8
    }

    #[test]
    fn multiply_does_not_have_the_downward_bias_of_the_shift_trick() {
        // Pins the reason this crate does not use `(v + (v >> 8) + 1) >> 8`:
        // it is `floor`, not `round`, so it drifts darker on every application.
        let mut differ = 0u32;
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                let truncated = floor_div_255(u32::from(a) * u32::from(b));
                let rounded = u32::from(multiply_u8(a, b));

                // The bias is one-directional: truncation is never larger.
                assert!(truncated <= rounded, "a={a} b={b}");
                if truncated != rounded {
                    differ += 1;
                }
            }
        }
        assert!(differ > 0, "the two conventions cannot be identical");
    }

    #[test]
    fn screen_is_the_complement_of_multiply_for_every_input() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                assert_eq!(
                    screen_u8(a, b),
                    255 - multiply_u8(255 - a, 255 - b),
                    "a={a} b={b}"
                );
            }
        }
    }

    #[test]
    fn screen_endpoints_are_exact_and_it_only_lightens() {
        for c in 0..=255u8 {
            assert_eq!(screen_u8(c, 0), c, "black is not the identity for {c}");
            assert_eq!(screen_u8(c, 255), 255, "white did not saturate {c}");
            for b in 0..=255u8 {
                assert!(screen_u8(c, b) >= c, "screen darkened c={c} b={b}");
            }
        }
    }

    #[test]
    fn mix_endpoints_are_exact_for_every_pair() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                assert_eq!(mix_u8(a, b, 0), a, "t=0 moved a={a} b={b}");
                assert_eq!(mix_u8(a, b, 255), b, "t=255 fell short a={a} b={b}");
            }
        }
    }

    #[test]
    fn mix_is_exact_round_to_nearest_for_every_input() {
        // 16.7M iterations of integer math; ~1s in a debug build, which is the
        // price of proving the invariant rather than sampling it.
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                let (ai, bi) = (i32::from(a), i32::from(b));
                for t in 0..=255u8 {
                    let expected = ai + round_div_255((bi - ai) * i32::from(t));
                    assert_eq!(i32::from(mix_u8(a, b, t)), expected, "a={a} b={b} t={t}");
                }
            }
        }
    }

    #[test]
    fn mix_rounds_symmetrically_in_both_directions() {
        for a in 0..=255u8 {
            for b in a..=255u8 {
                for t in [1u8, 17, 64, 128, 191, 254] {
                    let up = i32::from(mix_u8(a, b, t)) - i32::from(a);
                    let down = i32::from(b) - i32::from(mix_u8(b, a, t));
                    assert_eq!(up, down, "a={a} b={b} t={t}");
                }
            }
        }
    }

    #[test]
    fn mix_stays_between_its_endpoints() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
                for t in [0u8, 1, 63, 127, 128, 200, 254, 255] {
                    let m = mix_u8(a, b, t);
                    assert!((lo..=hi).contains(&m), "a={a} b={b} t={t} m={m}");
                }
            }
        }
    }

    #[test]
    fn ops_are_usable_in_const_contexts() {
        const M: u8 = multiply_u8(200, 128);
        const S: u8 = screen_u8(200, 128);
        const X: u8 = mix_u8(0, 254, 128);
        assert_eq!((M, S, X), (100, 228, 127));
    }
}
