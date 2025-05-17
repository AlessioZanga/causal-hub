#![warn(missing_docs)]
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

/// Assets such as datasets, models, and other resources.
pub mod assets;
/// Dataset structures.
pub mod datasets;
/// Probability distributions.
pub mod distributions;
/// Estimators for parameters and structures.
pub mod estimators;
/// Graph structures and algorithms.
pub mod graphs;
/// Input/output.
pub mod io;
/// Models.
pub mod models;
/// Random data generators.
pub mod random;
/// Sampling methods.
pub mod samplers;
/// Types.
pub mod types;
/// Utilities functions.
pub mod utils;
