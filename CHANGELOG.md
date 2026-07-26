# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-07-26

### Added

- **`gem::channel`**: `const fn` integer channel arithmetic that needs no math backend and works
  in a `const` context. An 8-bit channel is treated as fixed point, where `255` is `1.0`.
  - `multiply_u8(a, b)` — `a * b / 255`, the W3C `multiply` blend mode and equally the
    premultiply-by-alpha step. `255` is the exact identity and `0` the exact annihilator.
  - `screen_u8(a, b)` — the exact complement of `multiply_u8`.
  - `mix_u8(a, b, t)` — interpolation with an integer `t`, exact at both endpoints and
    symmetric under rounding in either direction.
- **`gem::rgb::distance_sq`**: `const fn` squared euclidean distance over `(u8, u8, u8)`, for
  nearest-color palette searches that want to stay in integer math.
- **`const fn` operations on `Rgb888` and `Bgr888`**: `multiply`, `screen`, `mix_u8`, and
  `distance_sq`, built on the primitives above. These are matched by *color*, not by storage
  order, so a `Bgr888` and an `Rgb888` holding the same color give the same answer.
- **`const fn` accessors**: `Rgb::to_rgb`, `Bgr::to_bgr`, and `Bgr::to_rgb`. `RgbColor::to_rgb`
  is a trait method and so is not callable from a `const fn` on stable; these inherent methods
  are, which is what makes a `const fn` color pipeline over gem's own types possible instead of
  over bare tuples.
- **`Mix::mix_assign`**, the in-place counterpart of `Mix::mix`.
- A documented, crate-wide rounding invariant: every integer channel result is round-to-nearest,
  ties away from zero. `gem::channel`'s tests prove it exhaustively over all 65 536 input pairs
  (and all 16.7M `(a, b, t)` triples for `mix_u8`) rather than by sampling, including a test
  pinning why the cheap truncating `(v + (v >> 8) + 1) >> 8` convention is *not* used: it is
  exactly `floor(v / 255)`, so it drifts a channel darker on every application.

### Changed

- **`Lerp` and `LerpChannel` are now `Mix` and `MixChannel`**, and `Mix` has moved to the crate
  root because it now spans both layers: `gem::Mix` is implemented by every RGB pixel format
  *and* by `Srgb`, `LinearRgb`, `Hsl`, `Hsv`, `Oklab`, and `Oklch`. `MixChannel` lives in the new
  `gem::channel` module alongside the integer math it is the real-valued counterpart of.

  The inherent `lerp` methods on the color-space types are gone; use the trait method. `mix` also
  reads correctly next to `mix_u8`, where two functions both called `lerp` with different `t`
  domains would not.

  ```rust,ignore
  // Before                              // After
  use gem::rgb::Lerp;                    use gem::Mix;
  a.lerp(b, 0.5);                        a.mix(b, 0.5);
  Srgb::RED.lerp(Srgb::BLUE, 0.5);       Srgb::RED.mix(Srgb::BLUE, 0.5);
  gem::blend::lerp(..);                  gem::blend::mix(..);
  ```

- **`RgbColor::into_rgb` and `RgbaColor::into_rgba` are now `to_rgb` and `to_rgba`.** Every color
  type in this crate is `Copy`, so nothing was ever consumed; `into_` was the wrong convention
  per the Rust API guidelines.

- **`space`, `named`, and `blend` are now `#[cfg]`-gated on `std` or `libm`** rather than always
  compiled with a `compile_error!` if neither is present. This is the fix for the underlying
  problem `0.1.0` worked around by setting `default = ["libm"]`, so **`default = []` is restored**
  and a bare `cargo add gem` now gets the pixel-format layer alone.

  Consumers who relied on the default feature set to get `space` must now name `std` (or `libm`)
  explicitly. Consumers who want only zero-cost pixel formats no longer need `default-features =
  false` — and if they set it anyway, `space`'s absence is now a normal missing module rather than
  a hard compile error.

  A new `space` feature exists to express "either backend"; `std`, `libm`, and `blend` all imply
  it, and naming it directly without a backend is still a compile error with an actionable
  message.

## [0.1.1] - 2026-07-04

### Fixed

- docs.rs build was failing: `#![feature(doc_auto_cfg)]` was merged into `doc_cfg` and removed as
  of the nightly toolchain docs.rs builds with (rustc 1.92+). Switched to `#![feature(doc_cfg)]`,
  the same fix already applied in `grixy` and `ixy` (`ixy` 0.6.1) for the identical reason.

## [0.1.0] - 2026-07-04

### Fixed

- **`cargo publish`'s own verification build failed**: `default = []` meant a bare `cargo add gem`
  (or `cargo publish`, which builds the default feature set to verify the package) failed to
  compile, because the always-compiled `space` module hits a `compile_error!` in `src/space/math.rs`
  without `std` or `libm` enabled. This shipped in the `v0.1.0` tag's first publish attempt, which
  failed in CI before anything reached crates.io. Fixed by making `default = ["libm"]`; consumers
  who only want zero-cost pixel formats can still `default-features = false`. Added `cargo build`
  (default features) to `just check` so this class of regression fails CI immediately instead of
  only surfacing at publish time.
- `named` module doc comment claimed "147" CSS named colors while only defining 141 `const`s.
  Added the 7 missing British-spelling aliases (`GREY`, `DARK_GREY`, `DARK_SLATE_GREY`,
  `DIM_GREY`, `LIGHT_GREY`, `LIGHT_SLATE_GREY`, `SLATE_GREY`) so the crate now defines all 148
  CSS Color Module Level 4 named colors, and corrected the doc count.

### Changed

- **License changed from `MIT` to `MIT OR Apache-2.0`**, matching every other crate in the
  ecosystem (`ixy`, `grixy`, `hexal`, `framepace`). Added `LICENSE-APACHE` and renamed `LICENSE` to
  `LICENSE-MIT`.
- **MSRV lowered from `1.88` to `1.87`**, matching the rest of the ecosystem. Verified the crate
  builds, lints, and tests clean on `1.87` with `--all-features`; nothing in the source actually
  required `1.88`.

### Chores

- `just semver-checks` no longer hardcodes a stale prerelease baseline (`0.1.0-alpha.4`); lets
  `cargo-semver-checks` auto-select the latest published baseline, matching `grixy`'s fix for the
  same issue.

## [0.1.0-alpha.7] - 2026-07-03

### Changed (breaking)

- **Fixed asymmetric/missing pixel format <-> `Srgb` conversions.** Every
  pixel format (`Rgb888`, `Bgr888`, `Rgb565`, `Argb1555`, `Argb4444`,
  `Argb8888`, `Abgr8888`, `RgbF32`, `RgbaF32`, and custom `Rgb<T>`/`Bgr<T>`)
  now converts to and from `Srgb` correctly, scaled for its channel bit
  depth. Previously `Abgr8888`/`Argb8888` only converted *to* `Srgb`, and
  most other formats had no space conversion at all. See `space::channel`
  (`RgbChannelScale`, `FromSrgb`) for the mechanism, and implement it for
  your own `RgbColor` types to opt in.
- **`Rgbf32`/`Rgbaf32` renamed to `RgbF32`/`RgbaF32`** for casing consistency
  with `GrayF32`.
- **`Rgb888`/`Bgr888` doc corrected**: these types have no padding
  (`size_of == 3`, `align_of == 1`); the previous "8 bits padding" claim was
  wrong.
- **`blend::alpha_over` argument order changed** to `(src, src_alpha, dst,
  dst_alpha)`, matching the function name and the Porter-Duff / CSS Color 4
  convention (previously `dst` came first).
- **`blend::premultiply`/`unpremultiply` replaced by a `Premultiplied`
  type** (`Premultiplied::new`/`.straight()`) so premultiplied vs. straight
  alpha is a type-level distinction instead of a `[f32; 4]` convention.
- **`default` features changed from `["libm", "libm-arch"]` to `[]`.** The
  pixel-format layer never needed a math backend; `space` still requires
  `std` or `libm` and fails fast with a clear error if neither is enabled.

### Added

- `space::{ToLinear, ConvertSpace}`: a hub-and-spoke conversion trait so any
  two color spaces implementing `ToLinear` can convert via
  `.convert::<T>()`, even without a hand-written `From` impl between them.
- `alpha::WithAlpha`: a generic `.with_alpha_first()`/`.with_alpha_last()`
  trait for attaching an alpha channel to any `Copy` color, unifying what
  were previously two disconnected mechanisms (packed-bit-field macro vs.
  manual `AlphaFirst`/`AlphaLast` wrapping).
- `define_packed_rgb!`/`define_packed_argb!` internal macros generating
  packed pixel formats (`Rgb565`, `Argb1555`, `Argb4444`) from a bit-layout
  table instead of ~150 lines of hand-written, occasionally-drifting
  boilerplate per format.
- Optional `serde` feature: `Serialize`/`Deserialize` for every pixel format
  and color space type.
- "Non-goals" section in the crate root docs.

## [0.1.0-alpha.6] - 2026-07-02

### Added

- **Per-channel interpolation**: `Lerp` and `LerpChannel` traits in the `rgb`
  module (also re-exported from the prelude). `Lerp` is blanket-implemented for
  every `RgbColor` whose channels implement `LerpChannel` — `Rgb888`, `Rgb565`,
  `Bgr888`, `Rgbf32`, the ARGB/ABGR types, and custom `Rgb<T>`/`Bgr<T>`.
  Interpolation happens in each format's native channel domain (no `Srgb`
  round-trip): integer channels round half away from zero and stay exact at
  `t = 0.5` (e.g. `0, 255 -> 128`), floating-point channels interpolate without
  clamping. Only the red, green, and blue channels are interpolated; alpha and
  padding bits are copied from the first operand. Requires neither `std` nor
  `libm`.

## [0.1.0-alpha.5] - 2026-06-26

### Added

- **Color spaces**: `Srgb`, `LinearRgb`, `Hsl`, `Hsv`, `Oklab`, `Oklch` —
  per-channel `f32` types with `From` conversions between all spaces.
  `Oklch` requires the `std` or `libm` feature for trigonometric functions.
- **Blending**: `alpha_over`, `premultiply`, `unpremultiply`, `lerp`
  functions in the new optional `blend` module (gated behind `features = ["blend"]`).
  Compatible with the standalone `alpha-blend` crate for full Porter-Duff compositing.
- **Named colors**: All ~147 CSS named color constants in the `named` module.
- **Conversions**: Pixel format types (`Rgb888`, `Bgr888`, `Abgr8888`,
  `Argb8888`) can now convert to/from `Srgb`.
- `From<u32>` / `From<Self> for u32` for packed integer types.
- `Hash` derive on all integer-component pixel format types.
- `UpperHex`, `LowerHex`, `Display` impls for integer formats.
- `From<[u8; N]>` / `From<T> for [u8; N]` for `Rgb888`, `Bgr888`.
- `std` and `libm` feature flags for float math support in `no_std`
  environments (follows the `alpha-blend` pattern).

### Changed

- **New lint configuration**: now matches `rg` — `unsafe_code = "deny"`,
  `clippy::all`, `pedantic`, and `nursery` all denied.
- `AlphaFirst` and `AlphaLast` docs clarified to show memory-order
  vs. constructor argument order.
- Fixed nightly-only doc attribute.
- Updated README to show correct API (`from_abgr` instead of `new_red`).

## [0.1.0-alpha.3] - 2025-07-29

### Added

- Exported `HasAlpha` and `HasGray` traits in the prelude
- Added `RgbaColor`, and implement it for compatible types
- Added `scalar` module to the public API

### Changed

- Crate now functions with or without either `std` or `libm` with a fallback

### Removed

- Removed `*::new_*` color constructors
- All functions around getting/setting scaled values have been removed
- `Normalize[d]` traits

## [0.1.0-alpha.2] - 2025-07-28

### Added

- Optional feature `bytemuck` that enables support for all structs in the crate
- Implemented forwarding `Has{*}` on `AlphaFirst` and `AlphaLast` structs

### Changed

- Renamed `With{*}` traits to `Has{*}`

## [0.1.0-alpha.1] - 2025-07-28

Initial release, claiming the `gem` crate name from an original author.
