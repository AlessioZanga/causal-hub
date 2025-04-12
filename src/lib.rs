#![warn(missing_docs)] // This will warn if any public items are missing documentation.
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

/// Assets such as datasets, models, and other resources.
pub mod assets;
/// Dataset structures.
pub mod dataset;
/// Probability distributions.
pub mod distribution;
/// Estimators for probability distributions.
pub mod estimator;
/// Graph structures and algorithms.
pub mod graph;
/// Input/output utilities.
pub mod io;
/// Models and algorithms for inference.
pub mod model;
/// Sampling methods for models.
pub mod sampler;
/// Helper types.
pub mod types;
/// Utilities and helper functions.
pub mod utils;
