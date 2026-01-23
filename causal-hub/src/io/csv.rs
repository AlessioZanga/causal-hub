use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use crate::types::Result;

/// A trait for reading and writing CSV files.
pub trait CsvIO: Sized {
    /// Create an instance of the type from a CSV reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - The CSV reader to read from.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_csv_reader<R: Read>(reader: R) -> Result<Self>;

    /// Write the instance to a CSV writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The CSV writer to write to.
    ///
    fn to_csv_writer<W: Write>(&self, writer: W) -> Result<()>;

    /// Create an instance of the type from a CSV string.
    ///
    /// # Arguments
    ///
    /// * `csv` - The CSV string to parse.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_csv_string(csv: &str) -> Result<Self> {
        // Read from the string as buffer.
        Self::from_csv_reader(csv.as_bytes())
    }

    /// Convert the instance to a CSV string.
    ///
    /// # Returns
    ///
    /// A CSV string representation of the instance.
    ///
    fn to_csv_string(&self) -> Result<String> {
        // Create a buffer to write to.
        let mut buffer = Vec::new();
        // Write to the buffer.
        self.to_csv_writer(&mut buffer)?;
        // Convert the buffer to a string.
        String::from_utf8(buffer).map_err(Into::into)
    }

    /// Create an instance of the type from a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the CSV file to read.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_csv_file(path: &str) -> Result<Self> {
        // Open the CSV file.
        let file = File::open(path)?;
        // Create a buffered reader.
        let reader = BufReader::new(file);
        // Read from the reader.
        Self::from_csv_reader(reader)
    }

    /// Write the instance to a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the CSV file to write.
    ///
    fn to_csv_file(&self, path: &str) -> Result<()> {
        // Create a file to write to.
        let file = File::create(path)?;
        // Create a buffered writer.
        let writer = BufWriter::new(file);
        // Write to the file.
        self.to_csv_writer(writer)
    }
}
