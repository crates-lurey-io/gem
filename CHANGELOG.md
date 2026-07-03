# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
