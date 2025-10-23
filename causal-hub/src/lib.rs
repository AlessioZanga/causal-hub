#![warn(missing_docs)]
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

// Link to the BLAS library.
#[cfg(any(feature = "openblas-static", feature = "openblas-system"))]
extern crate blas_src;

/// Assets such as datasets, models, and other resources.
pub mod assets;
/// Dataset structures.
pub mod datasets;
/// Parameter and structure learning algorithms.
pub mod estimators;
/// Inference algorithms.
pub mod inference;
/// Input/output functions.
pub mod io;
/// Models structures.
pub mod models;
/// Random generators.
pub mod random;
/// Sampling algorithms.
pub mod samplers;
/// Support types.
pub mod types;
/// Utility functions.
pub mod utils;
