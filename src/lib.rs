#![warn(missing_docs)] // This will warn if any public items are missing documentation.
//! # CausalHub
//!
//! CausalHub is a library for causal inference and causal discovery.
//! It provides tools for estimating causal effects, learning causal structures, and more.

/// Assents such as datasets, models, and other resources.
pub mod assets;
/// Data structures.
pub mod data;
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
/// Helper types and functions.
pub mod types;
