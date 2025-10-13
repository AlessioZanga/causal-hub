# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## v0.0.2 - Unreleased

### Added

* Added `cargo` test and release workflows.
* Added `maturin` build system for Python bindings.
* Added `Arc<RwLock<...>>` wrapping to reduce memory allocation and allow concurrency.

### Changed

* Changed regularization term for Cholesky decomposition.

## v0.0.1 - 2025-10-09

### Added

* Initial release.
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
