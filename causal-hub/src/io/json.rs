use std::io::{Read, Write};

/// A trait for reading and writing JSON files.
pub trait JsonIO {
    /// Create an instance of the type from a JSON reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - The reader to read from.
    ///
    /// # Returns
    ///
    /// An instance of the type.
    ///
    fn from_json_reader<R: Read>(reader: R) -> Self;

    /// Write the instance to a JSON writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to write to.
    ///
    fn to_json_writer<W: Write>(&self, writer: W);

    /// Create an instance of the type from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON string to parse.
    ///
    /// # Returns
    ///
    /// An instance of the type.
    ///
    fn from_json_string(json: &str) -> Self;

    /// Convert the instance to a JSON string.
    ///
    /// # Returns
    ///
    /// A JSON string representation of the instance.
    ///
    fn to_json_string(&self) -> String;

    /// Create an instance of the type from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file.
    ///
    /// # Returns
    ///
    /// An instance of the type.
    ///
    fn from_json_file(path: &str) -> Self;

    /// Write the instance to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file.
    ///
    fn to_json_file(&self, path: &str);
}

/// A macro to implement the `JsonIO` trait for a given type.
#[macro_export]
macro_rules! impl_json_io {
    ($type:ty) => {
        impl $crate::io::JsonIO for $type {
            fn from_json_reader<R: std::io::Read>(reader: R) -> Self {
                // Create a buffered reader.
                let reader = std::io::BufReader::new(reader);
                // Parse the JSON string.
                let json = serde_json::from_reader(reader)
                    .expect("Failed to parse JSON from reader.");
                // Get the JSON Schema id.
                let id = concat!(
                    paste::paste! { stringify!([<$type:lower>]) },
                    ".schema.json"
                );
                // Load the JSON Schema validator.
                let validator = jsonschema::options()
                    .with_retriever(&*$crate::assets::JSON_SCHEMA_RETRIEVER)
                    .build(&serde_json::json!({"$ref": id}))
                    .expect("Failed to build JSON Schema validator.");
                // Validate the JSON against the schema.
                validator
                    .validate(&json)
                    .expect("Failed to validate JSON against schema.");
                // Convert the parsed JSON to the type.
                serde_json::from_value(json)
                    .expect("Failed to convert JSON to type.")
            }

            fn to_json_writer<W: std::io::Write>(&self, writer: W) {
                // Create a buffered writer.
                let writer = std::io::BufWriter::new(writer);
                // Write the JSON to the writer.
                serde_json::to_writer(writer, self)
                    .expect("Failed to write JSON to writer.");
            }

            fn from_json_string(json: &str) -> Self {
                // Parse the JSON string.
                Self::from_json_reader(json.as_bytes())
            }

            fn to_json_string(&self) -> String {
                // Create a buffer.
                let mut buffer = Vec::new();
                // Write the JSON to the buffer.
                self.to_json_writer(&mut buffer);
                // Convert the buffer to a string.
                String::from_utf8(buffer)
                    .expect("Failed to convert JSON to string.")
            }

            fn from_json_file(path: &str) -> Self {
                // Open the file.
                let file = std::fs::File::open(path)
                    .expect("Failed to open JSON file.");
                // Create a buffered reader.
                let reader = std::io::BufReader::new(file);
                // Parse the JSON string.
                Self::from_json_reader(reader)
            }

            fn to_json_file(&self, path: &str) {
                // Create the file.
                let file = std::fs::File::create(path)
                    .expect("Failed to create JSON file.");
                // Create a buffered writer.
                let writer = std::io::BufWriter::new(file);
                // Write the JSON to the file.
                self.to_json_writer(writer);
            }
        }
    };
}
