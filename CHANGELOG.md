# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
