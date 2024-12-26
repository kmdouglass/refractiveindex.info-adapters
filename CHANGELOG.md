# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `retain` and `remove_many` methods to the `Store`.

## [0.2.0] 2024-12-19

### Added

- Added `get`, `insert`, `keys`, and `remove` methods to the `Store` for accessing its data.
- An `interpolate` method to `DispersionData` to evaluate the dispersion curves at a given wavelength.
- `n` and `k` methods to `Material` to evaluate the database entry's complex refractive indexes at a given wavelength.

### Changed

- Exposed the `DispersionData` and `Material` structs that represent the data and entries inside a `Store`.
- The CLI tool and its dependencies are now an optional feature called `cli`.

## [0.1.0] 2024-12-16

### Added

- The `ria` command line tool for converting the RefractiveIndex.INFO YAML database into a single file containing a flat key/value store.

[Unreleased]: https://github.com/kmdouglass/refractiveindex.info-adapters/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/kmdouglass/refractiveindex.info-adapters/releases/tag/v0.2.0
[0.1.0]: https://github.com/kmdouglass/refractiveindex.info-adapters/releases/tag/v0.1.0
