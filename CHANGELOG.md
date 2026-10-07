# Changelog

<!-- markdownlint-disable-file MD024 -->

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

…

## [0.1.3] - 2026-10-07

### Changed

* Minimum supported Rust version raised from 1.56.1 to 1.85, as required by current `clap` and `brotli`.
* Updated dependencies: `brotli` 7.0.0 → 9.0.0, `clap` 4.5.47 → 4.6.7, `glob` 0.3.3 → 0.3.4.
* CI: GitHub Actions pinned to SHA refs, Dependabot and zizmor analysis added.

## [0.1.2] - 2025-09-09

### Fixed

* Fixed clippy warnings related to MSRV compatibility by replacing `default_values_t` with manual default handling.

## [0.1.1] - 2025-04-10

* Updated dependencies and copyright notices.

### Added

* Release archives now contain `CHANGELOG.md`.

### Removed

* Spurious `doc` directory in release archives.

## [0.1.0] - 2022-01-25

Initial public release of **furnel**.

[Unreleased]: https://github.com/pyxy-dk/furnel/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/pyxy-dk/furnel/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/pyxy-dk/furnel/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/pyxy-dk/furnel/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/pyxy-dk/furnel/releases/tag/v0.1.0
