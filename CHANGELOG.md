# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html):

    ## Release Version - Release Date

    ### Added
    ### Changed
    ### Deprecated
    ### Removed
    ### Fixed
    ### Security

Begin of the actual Changelog:

## [Unreleased] 0.2.0 - 2023-03-17

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## 0.1.2 - 2023-02-24

### Added

- Added `CHANGELOG`, `CITATION`, `CODE_OF_CONDUCT`, `CONTRIBUTING` docs.
- Added issues templates and pull request templates.
- Added `docoverage` and `sort` workflows.
- Added `Correlation*`, `Covariance*` and `PrecisionMatrix`.
- Added `PartialCorrelation` statistic.
- Added `ChiSquared`, `FisherZ`, `StudentsT` conditional independence tests.
- Added `ConfusionMatrix` and `SHD` metrics.
- Added `From<DOT>` for `Graph` and `DiGraph`.
- Added `with_shuffle` method for `HillClimbing`.

### Changed

- Updated `RENAME`.
- Changed Rust setup action to `dtolnay/rust-toolchain`.
- Changed workflows triggers to `[push, pull_request]`.

### Fixed

- Fixed invalid edge reversal in `HillClimbing`.

## 0.1.1 - 2023-02-03

### Added

- Added `DOT` parser and plotter.

### Changed

## 0.1.0 - 2023-01-23

### Added

- Added `AcyclicGraph`, `SubGraph` trait.
- Added `UnionFind` structure.
- Added `ConnectedComponents` graph algorithm.
- Added `GraphicalSeparation` graph algorithm.
- Added `Categorical*` and `ContinuousDataset` data structure.
- Added `*CountMatrix` data structure.
- Added `LogLikelihood`, `Akaike*`, `BayesianInformationCriterion` statistics, for both categorical and Gaussian distributions.
- Added `HillClimbing` causal discovery algorithm.
- Added `ForbiddenRequired` edge lists structure.

### Changed

- Updated CI/CD linting stages.
- Updated CI/CD workflows.
- Updated vertex label getters.
- Updated `BFS`, `DFSEdges` implementation.

## 0.0.3 - 2023-01-06

### Added

- Added `BFS`, `DFS` graph algorithms.

## 0.0.2 - 2023-01-05

### Added

- Added CI/CD workflows.
- Added `DirectedGraph` trait.
- Added `DirectedDenseMatrix` graph.

## 0.0.1 - 2022-11-28

### Added

- Added `Rust` project structure.
- Added `BaseGraph`, `DefaultGraph`, `PartialOrdGraph` traits.
- Added `UndirectedDenseMatrix` graph.
