# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - YYYY-MM-DD

### Fixed

- Build error at core enviroment.

### Changed

- Polish etc.

## [0.3.0] - 2026-06-24

### Fixed

- `ArrDeque::clone` (NG: bit copy, not clone)
- `ArrDeque::drop` and `Drain::drop` (NG: double free) 

## [0.2.0] - 2026-06-18

### Fixed

- `ArrDeque::remove` (Double free)

## [0.1.2] - 2026-06-02

- Polish documentations.

## [0.1.1] - 2026-06-02

- Polish documentations.

## [0.1.0] - 2026-06-01

### Added

- Initial release.

[Unreleased]: https://github.com/nossie531/arr_deque/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/nossie531/arr_deque/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/nossie531/arr_deque/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/nossie531/arr_deque/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/nossie531/arr_deque/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/nossie531/arr_deque/releases/tag/v0.1.0
