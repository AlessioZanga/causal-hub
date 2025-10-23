/// A trait for reading and writing CSV files.
pub trait CsvIO: Sized {
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
    fn from_csv(csv: &str) -> Self;

    /// Convert the instance to a CSV string.
    ///
    /// # Returns
    ///
    /// A CSV string representation of the instance.
    ///
    fn to_csv(&self) -> String;

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
    fn read_csv(path: &str) -> Self {
        // TODO: Reading the entire file to a string is not efficient.
        Self::from_csv(&std::fs::read_to_string(path).expect("Failed to read CSV file."))
    }

    /// Write the instance to a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the CSV file to write.
    ///
    fn write_csv(&self, path: &str) {
        std::fs::write(path, self.to_csv()).expect("Failed to write CSV file.");
    }
}
