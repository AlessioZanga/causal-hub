#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(unstable_name_collisions)]

//! A hub for Causal Data Science.

/// Data processing module.
pub mod data;
/// Causal discovery module.
pub mod discovery;
/// Graph structures and algorithms module.
pub mod graphs;
/// Input/Output module.
pub mod io;
/// Probabilistic graphical models module.
pub mod models;
/// Plotting module.
pub mod plots;
/// Common structs and traits module.
pub mod prelude;
/// Statistics module.
pub mod stats;
/// Specialized types module.
pub mod types;
/// Utility module.
#[allow(unused_imports)]
pub mod utils;

pub use polars;
