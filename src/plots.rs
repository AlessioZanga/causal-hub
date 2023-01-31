use std::path::Path;

/// Plot trait.
pub trait Plot {
    /// Plot success type.
    type Success;
    /// Plot error type.
    type Error;

    /// Plot to given path.
    fn plot(self, path: &Path) -> Result<Self::Success, Self::Error>;
}
