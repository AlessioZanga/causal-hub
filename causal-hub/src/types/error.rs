use thiserror::Error;

/// An enum representing custom errors.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {}
