use std::path::PathBuf;

pub trait File: Into<String> + TryFrom<String> {
    type ReadError;

    type WriteError;

    fn read<P>(path: P) -> Result<Self, Self::ReadError>
    where
        P: Into<PathBuf>;

    fn write<P>(self, path: P) -> Result<(), Self::WriteError>
    where
        P: Into<PathBuf>;
}
