#![warn(missing_docs)]
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

/// Assets such as datasets, models, and other resources.
pub mod assets;
/// Dataset structures.
pub mod datasets;
/// Parameter and structure learning algorithms.
pub mod estimation;
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
