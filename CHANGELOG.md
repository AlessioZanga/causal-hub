# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## v0.0.4 - 2026-01-14

### Added

* Added `MissingTable` and `CatIncTable` for missing values in categorical data.
* Added `list-wise` and `pair-wise` deletion for `CatIncTable`.
* Added `IPW` (Inverse Probability Weighting) and `aIPW` (Augmented IPW) implementations for causal inference.
* Added `SSE` implementation for `CatIncTable`.
* Added `to_csv` implementation for `CatTable` and `GaussTable`.
* Added `CsvIO` for `CatIncTable`.
* Added `accelerate` as BLAS backend on macOS.
* Added Linux ARM64 support.
* Added `Dataset` superclass for Python frontend.

### Changed

* Refactored project structure to improve parameter estimators implementation.
* Refactored I/O traits.
* Refactored `debug_asserts` usage and `states` sorting.
* Updated default features.

### Fixed

* Fixed Python documentation build.
* Fixed Windows multi-line commands in workflows.

## v0.0.3 - 2025-12-11

### Fixed

* Fixed CACE estimation when the minimal backdoor adjustment set is exactly equal to the conditioning set.
* Fixed pyproject configuration for maturin build.

## v0.0.2 - 2025-10-23

### Added

* Added `cargo` test and release workflows.
* Added `maturin` build system for Python bindings.
* Added `Arc<RwLock<...>>` wrapping to reduce memory allocation and allow concurrency.
* Added `PseudoInverse` trait for matrix pseudo-inversion.

### Fixed

* Fixed `GaussCPD` fit stability with SVD.
* Fixed `GaussPhi` implementation.

## v0.0.1 - 2025-10-09

### Added

* Added initial release.
* Added `README.md` file.
* Added `CHANGELOG.md` file.
* Added `LICENSE` file.
* Added `CatTable` data structure.
* Added `GaussTable` data structure.
* Added `CatTrj` data structure.
* Added `CatBN` model.
* Added `CatCPD` parameters.
* Added `GaussBN` model.
* Added `GaussCPD` parameters.
* Added `CatCTBN` model.
* Added `CatCIM` parameters.
* Added `DiGraph` structure.
* Added `UnGraph` structure.
* Added `PK` structure for prior knowledge.
* Added `MLE` and `BE` estimators for all the above.
* Added `EM` and `SEM` estimators for all the above.
* Added `CTPC` and `CTHC` algorithms.
* Added `PyO3` bindings for all the above.
