#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(unstable_name_collisions)]

//! A hub for Causal Data Science.

pub mod data;

pub mod discovery;

pub mod graphs;

pub mod io;

pub mod models;

pub mod plots;

pub mod prelude;

pub mod stats;

pub mod types;

#[allow(unused_imports)]
pub mod utils;

pub use polars;
