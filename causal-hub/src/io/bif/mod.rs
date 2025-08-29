mod parser;
pub use parser::BifParser;

/// A trait for reading and writing BIF files.
pub trait BifIO {
    /// Create an instance of the type from a BIF string.
    ///
    /// # Arguments
    ///
    /// * `bif` - A string slice that holds the BIF data.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_bif(bif: &str) -> Self;

    /// Convert the instance to a BIF string.
    ///
    /// # Returns
    ///
    /// A string slice that holds the BIF data.
    ///
    fn to_bif(&self) -> String;

    /// Read a BIF file and create an instance of the type.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the BIF file.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn read_bif(path: &str) -> Self;

    /// Write the instance to a BIF file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the BIF file.
    ///
    fn write_bif(&self, path: &str);
}
