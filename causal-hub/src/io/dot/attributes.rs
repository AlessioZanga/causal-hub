//! Lightweight, model-agnostic DOT attributes.

use crate::types::Map;

/// Graph-level attributes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GraphAttributes(pub Map<String, String>);

/// Vertex (node) attributes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VertexAttributes(pub Map<String, String>);

/// Edge attributes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EdgeAttributes(pub Map<String, String>);

impl GraphAttributes {
    /// Insert a raw `key = value` pair, unquoting the value if necessary.
    #[inline]
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), unquote(value));
    }

    /// Get the value associated with a key, if any.
    #[inline]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}

impl VertexAttributes {
    /// Insert a raw `key = value` pair, unquoting the value if necessary.
    #[inline]
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), unquote(value));
    }
}

impl EdgeAttributes {
    /// Insert a raw `key = value` pair, unquoting the value if necessary.
    #[inline]
    pub fn insert_raw_parts(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), unquote(value));
    }
}

/// Quote a value if it contains spaces or special characters.
pub(crate) fn quote(stats: &str) -> String {
    if stats.is_empty() || stats.contains(' ') || stats.contains('"') {
        format!("\"{}\"", stats.replace('"', "\\\""))
    } else {
        stats.to_string()
    }
}

/// Remove surrounding double quotes and unescape `\"`.
pub(crate) fn unquote(stats: &str) -> String {
    let stats = stats.trim();
    if stats.len() >= 2 && stats.starts_with('"') && stats.ends_with('"') {
        stats[1..stats.len() - 1].replace("\\\"", "\"")
    } else {
        stats.to_string()
    }
}

impl From<GraphAttributes> for String {
    fn from(a: GraphAttributes) -> String {
        a.0.iter()
            .map(|(k, v)| format!("{k}={}", quote(v)))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl From<VertexAttributes> for String {
    fn from(a: VertexAttributes) -> String {
        a.0.iter()
            .map(|(k, v)| format!("{k}={}", quote(v)))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl From<EdgeAttributes> for String {
    fn from(a: EdgeAttributes) -> String {
        a.0.iter()
            .map(|(k, v)| format!("{k}={}", quote(v)))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
