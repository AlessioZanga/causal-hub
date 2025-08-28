/// A trait for reading and writing JSON files.
pub trait JsonIO {
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
    fn from_json(json: &str) -> Self;

    /// Convert the instance to a JSON string.
    ///
    /// # Returns
    ///
    /// A JSON string representation of the instance.
    ///
    fn to_json(&self) -> String;

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
    fn read_json(path: &str) -> Self;

    /// Write the instance to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file.
    ///
    fn write_json(&self, path: &str);
}

/// A macro to implement the `JsonIO` trait for a given type.
#[macro_export]
macro_rules! impl_json_io {
    ($type:ty) => {
        impl $crate::io::JsonIO for $type {
            fn from_json(json: &str) -> Self {
                // Parse the JSON string.
                let json = serde_json::from_str(json).unwrap();
                // Get the JSON Schema id.
                let id = concat!(paste::paste! { stringify!([<$type:lower>]) }, ".schema.json");
                // Load the JSON Schema validator.
                let validator = jsonschema::options()
                    .with_retriever(&*$crate::assets::JSON_SCHEMA_RETRIEVER)
                    .build(&serde_json::json!({"$ref": id}))
                    .unwrap();
                // Validate the JSON against the schema.
                validator.validate(&json).unwrap();
                // Convert the parsed JSON to the type.
                serde_json::from_value(json).unwrap()
            }

            fn to_json(&self) -> String {
                serde_json::to_string(self).unwrap()
            }

            fn read_json(path: &str) -> Self {
                use std::{fs::File, io::BufReader};
                // Open the file.
                let file = File::open(path).unwrap();
                // Create a buffered reader.
                let reader = BufReader::new(file);
                // Parse the JSON string.
                let json = serde_json::from_reader(reader).unwrap();
                // Get the JSON Schema id.
                let id = concat!(paste::paste! { stringify!([<$type:lower>]) }, ".schema.json");
                // Load the JSON Schema validator.
                let validator = jsonschema::options()
                    .with_retriever(&*$crate::assets::JSON_SCHEMA_RETRIEVER)
                    .build(&serde_json::json!({"$ref": id}))
                    .unwrap();
                // Validate the JSON against the schema.
                validator.validate(&json).unwrap();
                // Convert the parsed JSON to the type.
                serde_json::from_value(json).unwrap()
            }

            fn write_json(&self, path: &str) {
                use std::{fs::File, io::BufWriter};
                let file = File::create(path).unwrap();
                let writer = BufWriter::new(file);
                serde_json::to_writer(writer, self).unwrap()
            }
        }
    };
}
