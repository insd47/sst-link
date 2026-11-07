# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/insd47/sst-link/compare/v0.1.0...v0.2.0) - 2026-11-07

### Changed

- [**breaking**] move link shapes into sst and sst::aws ([#2](https://github.com/insd47/sst-link/pull/2))

## [0.1.0] - 2026-09-26

### Added

- `#[derive(Links)]` loads every SST link a struct declares in one call, keyed by link name through serde
- Shapes for `App`, `Bucket`, `Dynamo`, `Function`, `Realtime`, `Router`, and `Secret` (no `Debug`)
