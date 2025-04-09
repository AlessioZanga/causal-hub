use std::io::Read;

use csv::Reader;

/// A trait for reading CSV files.
pub trait FromCsvReader {
    /// Reads a CSV file.
    ///
    /// # Arguments
    ///
    /// * `reader` - A CSV reader from the `csv` crate.
    ///
    /// # Returns
    ///
    /// A new instance of the implementing type.
    ///
    /// # Notes
    ///
    /// CSV reader should trim input.
    ///
    fn from_csv_reader<R: Read>(reader: Reader<R>) -> Self;
}
