use std::io::{Read, Write};

use crate::types::Result;

/// A trait for reading and writing JSON files.
pub trait JsonIO: Sized {
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
    fn from_json_reader<R: Read>(reader: R) -> Result<Self>;

    /// Write the instance to a JSON writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to write to.
    ///
    fn to_json_writer<W: Write>(&self, writer: W) -> Result<()>;

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
    fn from_json_string(json: &str) -> Result<Self>;

    /// Convert the instance to a JSON string.
    ///
    /// # Returns
    ///
    /// A JSON string representation of the instance.
    ///
    fn to_json_string(&self) -> Result<String>;

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
    fn from_json_file(path: &str) -> Result<Self>;

    /// Write the instance to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file.
    ///
    fn to_json_file(&self, path: &str) -> Result<()>;
}

/// A macro to implement the `JsonIO` trait for a given type.
#[macro_export]
macro_rules! impl_json_io {
    ($type:ty) => {
        impl $crate::io::JsonIO for $type {
            fn from_json_reader<R: std::io::Read>(reader: R) -> $crate::types::Result<Self> {
                // Create a buffered reader.
                let reader = std::io::BufReader::new(reader);
                // Parse the JSON string.
                let json = serde_json::from_reader(reader)?;
                // Get the JSON Schema id.
                let id = concat!(
                    paste::paste! { stringify!([<$type:lower>]) },
                    ".schema.json"
                );
                // Load the JSON Schema validator.
                let validator = jsonschema::options()
                    .with_retriever(&*$crate::assets::JSON_SCHEMA_RETRIEVER)
                    .build(&serde_json::json!({"$ref": id}))
                    .map_err(|e| $crate::types::Error::Parsing(format!("Failed to build JSON Schema validator: {}", e)))?;
                // Validate the JSON against the schema.
                let errors: Vec<_> = validator.iter_errors(&json).collect();
                if !errors.is_empty() {
                    let msg = errors
                        .into_iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err($crate::types::Error::Parsing(format!(
                        "JSON Schema validation failed: {}",
                        msg
                    )));
                }
                // Convert the parsed JSON to the type.
                serde_json::from_value(json).map_err(Into::into)
            }

            fn to_json_writer<W: std::io::Write>(&self, writer: W) -> $crate::types::Result<()> {
                // Create a buffered writer.
                let writer = std::io::BufWriter::new(writer);
                // Write the JSON to the writer.
                serde_json::to_writer(writer, self)?;
                Ok(())
            }

            fn from_json_string(json: &str) -> $crate::types::Result<Self> {
                // Parse the JSON string.
                Self::from_json_reader(json.as_bytes())
            }

            fn to_json_string(&self) -> $crate::types::Result<String> {
                // Create a buffer.
                let mut buffer = Vec::new();
                // Write the JSON to the buffer.
                self.to_json_writer(&mut buffer)?;
                // Convert the buffer to a string.
                String::from_utf8(buffer).map_err(Into::into)
            }

            fn from_json_file(path: &str) -> $crate::types::Result<Self> {
                // Open the file.
                let file = std::fs::File::open(path)?;
                // Create a buffered reader.
                let reader = std::io::BufReader::new(file);
                // Parse the JSON string.
                Self::from_json_reader(reader)
            }

            fn to_json_file(&self, path: &str) -> $crate::types::Result<()> {
                // Create the file.
                let file = std::fs::File::create(path)?;
                // Create a buffered writer.
                let writer = std::io::BufWriter::new(file);
                // Write the JSON to the file.
                self.to_json_writer(writer)
            }
        }
    };
}
