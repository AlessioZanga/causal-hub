use std::path::PathBuf;

/// Plot trait.
pub trait Plot {
    /// Plot success type.
    type Success;
    /// Plot error type.
    type Error;

    /// Plot to given path.
    fn plot<P>(self, path: P) -> Result<Self::Success, Self::Error>
    where
        P: Into<PathBuf>;
}
