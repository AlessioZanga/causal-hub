/// A trait for reading and writing CSV files.
pub trait CsvIO {
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
    fn read_csv(path: &str) -> Self;

    /// Write the instance to a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the CSV file to write.
    ///
    fn write_csv(&self, path: &str);
}
