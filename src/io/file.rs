use std::path::PathBuf;

/// I/O file format trait.
pub trait File: Into<String> + TryFrom<String> {
    /// Read format from path error.
    type ReadError;
    /// Write format to path error.
    type WriteError;

    /// Read format from a given path.
    fn read<P>(path: P) -> Result<Self, Self::ReadError>
    where
        P: Into<PathBuf>;

    /// Write format to a given path.
    fn write<P>(self, path: P) -> Result<(), Self::WriteError>
    where
        P: Into<PathBuf>;
}
