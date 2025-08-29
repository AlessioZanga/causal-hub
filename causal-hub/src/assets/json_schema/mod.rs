use std::sync::LazyLock;

use dry::macro_for;
use jsonschema::{Retrieve, Uri};
use log::debug;
use serde_json::Value;

use crate::types::Map;

#[derive(Debug)]
pub(crate) struct InMemoryRetriever {
    schemas: Map<String, Value>,
}

impl Retrieve for &InMemoryRetriever {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        self.schemas
            .get(
                uri.as_str()
                    .strip_prefix("json-schema:///")
                    .unwrap_or_else(|| panic!("Failed to strip `json-schema:///` prefix")),
            )
            .cloned()
            .ok_or_else(|| format!("Schema not found: {uri}").into())
    }
}

pub(crate) static JSON_SCHEMA_RETRIEVER: LazyLock<InMemoryRetriever> = LazyLock::new(|| {
    // Log the loading of the JSON Schemas.
    debug!("Loading the JSON Schemas from assets.");
    // Allocate a map to hold the schemas.
    let mut schemas = Map::default();
    // Load all JSON Schemas.
    macro_for!(
        $schema in [
            catbn, catcim, catcpd, catctbn, digraph, ungraph
        ] {
        // Load the JSON Schema file.
        let schema_str = include_str!(concat!(stringify!($schema), ".schema.json"));
        let schema_json: Value = serde_json::from_str(schema_str).unwrap();
        // Get the $id of the schema, or use a default if not present.
        let id = schema_json
            .get("$id")
            .and_then(|v| v.as_str())
            .unwrap_or(concat!(stringify!($schema), ".schema.json"))
            .to_string();
        // Insert the schema into the map with its $id as the key.
        schemas.insert(id, schema_json);
    });
    // Create the retriever.
    InMemoryRetriever { schemas }
});
