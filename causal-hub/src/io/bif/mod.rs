mod parser;
pub use parser::BifParser;

use crate::types::Result;

/// A trait for reading and writing BIF files.
pub trait BifIO: Sized {
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
    fn from_bif_string(bif: &str) -> Result<Self>;

    /// Convert the instance to a BIF string.
    ///
    /// # Returns
    ///
    /// A string slice that holds the BIF data.
    ///
    fn to_bif_string(&self) -> Result<String>;

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
    fn from_bif_file(path: &str) -> Result<Self>;

    /// Write the instance to a BIF file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the BIF file.
    ///
    fn to_bif_file(&self, path: &str) -> Result<()>;
}
