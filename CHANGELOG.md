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

## v0.0.6 - 2026-08-24

### Added

* Added `MixedBN` model with mixed categorical-gaussian CPDs, potentials, datasets, and Python bindings.
* Added `HillClimbing` (`HC`) score-based structure learning algorithm.
* Added `HC`, `CTHC`, and `CTPC` structure learning wrappers in the Python frontend.
* Added `fit`, `fit_parameters`, and `fit_structure` methods for models and estimators.
* Added `DOT` and `GML` parsers for graph I/O.
* Added `CatWtdTable` and `GaussWtdTable` weighted datasets to the Python frontend.
* Added vertex manipulation methods for graphs.
* Added `Support` trait definition with `CatSupport` and `GaussSupport` implementations.

### Changed

* Updated structure estimators to return already fitted models.
* Updated default estimator for CTBNs.
* Updated `initial_graph` interface of structure estimators.
* Updated core dependencies.

### Removed

* Removed Windows support from CI workflows.

### Fixed

* Fixed Python documentation, Sphinx autodoc, and README badges.
* Fixed GitHub Actions deprecations and version SHA pinning.

## v0.0.5 - 2026-04-29

### Added

* Added `GaussIncTable` data structure.
* Added `MLE` and `BE` estimators for `GaussIncTable`.
* Added `IPW` and `aIPW` implementations for `GaussIncTable`.
* Added `SATE` (Sample Average Treatment Effect) and `PACE` (Population Average Treatment Effect) estimation.
* Added parallel implementation for `SATE` and gaussian statistics.
* Added `RngCatBN`, `RngGaussBN`, `RngCatCPD`, and `RngGaussCPD` for random model generation.
* Added `RngCatIncTable` and `RngGaussIncTable` for random incomplete dataset generation.
* Added `MissingMechanism` struct and support for incomplete dataset generation in Python frontend.
* Added `to_bif_string` implementation for Bayesian Networks.
* Added `InvalidParameter` and `IndexOutOfBounds` errors to replace generic exceptions.
* Added scoring criteria for model selection.
* Added `AGENTS.md` to guide AI-assisted development.

### Changed

* Refactored error handling to remove `unwrap` and `expect` calls across the workspace, using custom `Error` types.
* Updated `ApproximateInference` and `CausalInference` to support evidence and custom estimators.
* Updated `BE` (Bayesian Estimator) as the default for inference tasks.
* Updated `pyo3`, `rand`, and other core dependencies.
* Renamed `ACE` to `PACE` and `sample_*` methods to `fitted_*`.

### Fixed

* Improved numerical stability for `GaussCPD` and variance regularization.
* Fixed logical and log-likelihood checks.
* Fixed `SIGILL` error and various linting issues.
* Fixed Python documentation and test suite.

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
