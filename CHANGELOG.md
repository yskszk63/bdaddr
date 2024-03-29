# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0-alpha.4] - 2021-12-09

### Added

- Fix: Export `AddressType` struct.

## [0.2.0-alpha.3] - 2021-12-09

### Added

- `AddressType` struct.

## [0.2.0-alpha.2] - 2021-11-23

### Changed

- `RawAddress` (`Address` at v0.1.x) struct has been renamed to `BdAddr`.
- Added some conversion function between structs.

## [0.2.0-alpha.1] - 2021-11-20

### Added

- Github Actions Workflow.

### Changed

- SOME BREAKING CHANGES.
- Significantly revised the structure to clearly indicate the type (Classic / LE Public / LE Random).
- The old `Address` struct has been renamed to `RawAddress`.
- Change MSRV to 1.56 (2021 edition)

## [0.1.2] - 2021-09-23

### Changed

- Use Cargo `resolver` feature.

## [0.1.1] - 2021-07-11

### Changed

- Bump aes crate version.

## [0.1.0] - 2020-11-28

- Initial Release

[Unreleased]: https://github.com/yskszk63/bdaddr/compare/v0.2.0-alpha.4...HEAD
[0.2.0-alpha.4]: https://github.com/yskszk63/bdaddr/compare/v0.2.0-alpha.3...v0.2.0-alpha.4
[0.2.0-alpha.3]: https://github.com/yskszk63/bdaddr/compare/v0.2.0-alpha.2...v0.2.0-alpha.3
[0.2.0-alpha.2]: https://github.com/yskszk63/bdaddr/compare/v0.2.0-alpha.1...v0.2.0-alpha.2
[0.2.0-alpha.1]: https://github.com/yskszk63/bdaddr/compare/v0.1.2...v0.2.0-alpha.1
[0.1.2]: https://github.com/yskszk63/bdaddr/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/yskszk63/bdaddr/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/yskszk63/bdaddr/releases/tag/v0.1.0
