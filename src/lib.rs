#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(unstable_name_collisions)]

//! A hub for Causal Data Science.

/// Data algorithms and structures.
pub mod data;

/// Causal discovery algorithms and structures.
pub mod discovery;

/// Graphs algorithms and structures.
pub mod graphs;

/// I/O algorithms and structures.
pub mod io;

/// Models algorithms and structures.
pub mod models;

/// Plot algorithms and traits.
pub mod plots;

/// Frequently used items.
pub mod prelude;

/// Statistical module.
pub mod stats;

/// Crate-wide types.
pub mod types;

/// Crate-wide utils.
#[allow(unused_imports)]
pub mod utils;
