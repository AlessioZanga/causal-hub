#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! A hub for Causal Data Science.

/// Data algorithms and structures.
#[cfg(not(doctest))]
pub mod data;

/// Graphs algorithms and structures.
pub mod graphs;

/// Models algorithms and structures.
pub mod models;

/// Frequently used items.
pub mod prelude;

/// Crate-wide types.
pub mod types;

/// Crate-wide utils.
pub mod utils;
