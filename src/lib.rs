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
/// Estimators for probability distributions.
pub mod estimators;
/// Graph structures and algorithms.
pub mod graphs;
/// Input/output utilities.
pub mod io;
/// Models and algorithms for inference.
pub mod models;
/// Sampling methods for models.
pub mod samplers;
/// Helper types.
pub mod types;
/// Utilities and helper functions.
pub mod utils;
