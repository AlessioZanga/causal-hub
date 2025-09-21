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
            .get(uri.as_str())
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
            catcpd, catbn,
            gausscpd, gaussbn,
            catcim, catctbn,
            digraph, ungraph
        ] {
        // Load the JSON Schema file.
        let schema = include_str!(concat!(stringify!($schema), ".schema.json"));
        let schema: Value = serde_json::from_str(schema).unwrap();
        // Get the URI of the schema.
        let id = concat!("json-schema:///", stringify!($schema), ".schema.json");
        // Insert the schema into the map with its $id as the key.
        schemas.insert(id.to_owned(), schema);
    });
    // Create the retriever.
    InMemoryRetriever { schemas }
});
