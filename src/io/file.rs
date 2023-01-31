use std::path::Path;

/// I/O file format trait.
pub trait File: Into<String> + TryFrom<String> {
    /// Read format from path error.
    type ReadError;
    /// Write format to path error.
    type WriteError;

    /// Read format from a given path.
    fn read(path: &Path) -> Result<Self, Self::ReadError>;

    /// Write format to a given path.
    fn write(self, path: &Path) -> Result<(), Self::WriteError>;
}
