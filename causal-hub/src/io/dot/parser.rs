#![allow(missing_docs)]

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{
    io::dot::attributes::{EdgeAttributes, GraphAttributes, VertexAttributes, unquote},
    types::{Error, Map, Result},
};

/// A DOT parser built on top of `pest`.
///
/// The parsed representation is graph-agnostic: it stores the graph direction
/// (`graph_type`), an optional `strict` flag, an optional graph `id`, the
/// (ordered) vertex labels together with their attributes, and the list of
/// edges as label pairs (with attributes). Converting to a concrete graph type
/// (`DiGraph` / `UnGraph`) validates the directionality.
#[allow(missing_docs)]
#[derive(Parser)]
#[grammar = "src/io/dot/grammar.pest"]
pub struct DOTParser;

/// A graph-agnostic DOT representation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DOT {
    /// The graph direction (`"graph"` for undirected, `"digraph"` for directed).
    pub graph_type: String,
    /// Whether the `strict` keyword was present.
    pub strict: bool,
    /// The optional graph identifier.
    pub id: Option<String>,
    /// Graph-level attributes.
    pub graph_attributes: GraphAttributes,
    /// Default node attributes (`node [...]`).
    pub default_node_attributes: VertexAttributes,
    /// Default edge attributes (`edge [...]`).
    pub default_edge_attributes: EdgeAttributes,
    /// The set of vertex labels mapped to their attributes.
    pub vertices: Map<String, VertexAttributes>,
    /// The list of directed/undirected edges as `(from, to, attributes)` triples.
    pub edges: Vec<(String, String, EdgeAttributes)>,
}

impl DOT {
    /// Parse a DOT string into the graph-agnostic representation.
    ///
    /// # Arguments
    ///
    /// * `string` - The DOT content.
    ///
    /// # Returns
    ///
    /// The parsed [`DOT`] representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not valid DOT.
    ///
    pub fn from_string(string: &str) -> Result<Self> {
        let mut pairs = DOTParser::parse(Rule::file, string.trim())
            .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
        let pair = pairs
            .next()
            .ok_or_else(|| Error::Parsing("empty DOT document"))?;
        Self::from_pair(pair)
    }

    /// Build a [`DOT`] from a parsed `graph` pair.
    fn from_pair(pair: Pair<Rule>) -> Result<Self> {
        if pair.as_rule() != Rule::graph {
            return Err(Error::Parsing("expected a DOT graph"));
        }

        let mut strict = false;
        let mut graph_type = String::new();
        let mut id = None;
        let mut statements = None;

        for probability in pair.into_inner() {
            match probability.as_rule() {
                Rule::strict => strict = true,
                Rule::graph_type => graph_type = probability.as_str().to_string(),
                Rule::graph_id => {
                    id = probability.into_inner().next().map(|x| unquote(x.as_str()));
                }
                Rule::statements => statements = Some(probability),
                _ => {}
            }
        }

        let statements = statements.ok_or_else(|| Error::Parsing("missing DOT statements"))?;
        let (graph_attributes, default_node_attributes, default_edge_attributes, vertices, edges) =
            Self::parse_statements(statements)?;

        Ok(Self {
            graph_type,
            strict,
            id,
            graph_attributes,
            default_node_attributes,
            default_edge_attributes,
            vertices,
            edges,
        })
    }

    /// Walk the statement list, collecting vertices and edges.
    #[allow(clippy::type_complexity)]
    fn parse_statements(
        pair: Pair<Rule>,
    ) -> Result<(
        GraphAttributes,
        VertexAttributes,
        EdgeAttributes,
        Map<String, VertexAttributes>,
        Vec<(String, String, EdgeAttributes)>,
    )> {
        let mut graph_attributes = GraphAttributes::default();
        let mut default_node_attributes = VertexAttributes::default();
        let mut default_edge_attributes = EdgeAttributes::default();
        let mut vertices: Map<String, VertexAttributes> = Map::default();
        // Map from a vertex id (as written) to its resolved label.
        let mut id_to_label: Map<String, String> = Map::default();
        let mut edges: Vec<(String, String, EdgeAttributes)> = Vec::new();

        for stmt in pair.into_inner() {
            match stmt.as_rule() {
                Rule::attribute => {
                    let (k, v) = parse_attribute(stmt)?;
                    graph_attributes.insert_raw_parts(&k, &v);
                }
                Rule::global_attributes => {
                    let mut inner = stmt.into_inner();
                    let kind = inner
                        .next()
                        .ok_or_else(|| Error::Parsing("missing global attribute kind"))?;
                    let attrs = inner
                        .next()
                        .ok_or_else(|| Error::Parsing("missing global attributes"))?;
                    let attrs = parse_attributes(attrs)?;
                    match kind.as_str() {
                        "graph" => graph_attributes = GraphAttributes(attrs),
                        "node" => default_node_attributes = VertexAttributes(attrs),
                        "edge" => default_edge_attributes = EdgeAttributes(attrs),
                        _ => {}
                    }
                }
                Rule::vertex => {
                    let (raw_id, label, attrs) = parse_vertex(stmt)?;
                    id_to_label.insert(raw_id, label.clone());
                    vertices.insert(label, attrs);
                }
                Rule::path => {
                    let (ids, attrs) = parse_path(stmt)?;
                    let labels: Vec<String> = ids
                        .iter()
                        .map(|evidence| {
                            id_to_label
                                .get(evidence)
                                .cloned()
                                .unwrap_or_else(|| evidence.clone())
                        })
                        .collect();
                    // Nodes may be declared only through edges: ensure they exist.
                    for label in &labels {
                        vertices.entry(label.clone()).or_default();
                    }
                    for w in labels.windows(2) {
                        edges.push((w[0].clone(), w[1].clone(), attrs.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok((
            graph_attributes,
            default_node_attributes,
            default_edge_attributes,
            vertices,
            edges,
        ))
    }

    /// Serialize the [`DOT`] representation into a DOT string.
    pub(crate) fn to_string_repr(&self) -> Result<String> {
        let mut stats = String::new();

        if self.strict {
            stats.push_str("strict ");
        }
        stats.push_str(&self.graph_type);
        stats.push(' ');
        if let Some(id) = &self.id {
            stats.push_str(&quote_id(id));
            stats.push(' ');
        }
        stats.push_str("{\n");

        // Graph-level attributes.
        if !self.graph_attributes.0.is_empty() {
            stats.push_str(&format!(
                "\tgraph [{}];\n",
                String::from(self.graph_attributes.clone())
            ));
        }
        // Default node attributes.
        if !self.default_node_attributes.0.is_empty() {
            stats.push_str(&format!(
                "\tnode [{}];\n",
                String::from(self.default_node_attributes.clone())
            ));
        }
        // Default edge attributes.
        if !self.default_edge_attributes.0.is_empty() {
            stats.push_str(&format!(
                "\tedge [{}];\n",
                String::from(self.default_edge_attributes.clone())
            ));
        }

        // Vertices.
        for (label, attrs) in &self.vertices {
            let mut line = format!("\t\"{}\"", label.replace('"', "\\\""));
            if !attrs.0.is_empty() {
                line.push_str(&format!(" [{}]", String::from(attrs.clone())));
            }
            line.push_str(";\n");
            stats.push_str(&line);
        }

        // Edges.
        let op = if self.graph_type == "digraph" {
            "->"
        } else {
            "--"
        };
        for (a, b, attrs) in &self.edges {
            let mut line = format!(
                "\t\"{}\" {} \"{}\"",
                a.replace('"', "\\\""),
                op,
                b.replace('"', "\\\"")
            );
            if !attrs.0.is_empty() {
                line.push_str(&format!(" [{}]", String::from(attrs.clone())));
            }
            line.push_str(";\n");
            stats.push_str(&line);
        }

        stats.push_str("}\n");
        Ok(stats)
    }
}

/// Parse a single `attribute` rule into `(key, value)`.
fn parse_attribute(pair: Pair<Rule>) -> Result<(String, String)> {
    let mut inner = pair.into_inner();
    let key = inner
        .next()
        .ok_or_else(|| Error::Parsing("missing attribute key"))?
        .as_str()
        .to_string();
    let value = inner
        .next()
        .ok_or_else(|| Error::Parsing("missing attribute value"))?
        .as_str()
        .to_string();
    Ok((key, unquote(&value)))
}

/// Parse an `attributes` rule into a `key -> value` map.
fn parse_attributes(pair: Pair<Rule>) -> Result<Map<String, String>> {
    let mut map: Map<String, String> = Map::default();
    for attr in pair.into_inner() {
        let (k, v) = parse_attribute(attr)?;
        map.insert(k, v);
    }
    Ok(map)
}

/// Extract the (raw) id from a `vertex_id` rule.
fn vertex_id_string(pair: Pair<Rule>) -> Result<String> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| Error::Parsing("missing vertex id"))?;
    Ok(unquote(inner.as_str()))
}

/// Parse a `vertex` statement into `(raw_id, label, attributes)`.
fn parse_vertex(pair: Pair<Rule>) -> Result<(String, String, VertexAttributes)> {
    let mut inner = pair.into_inner();
    let vid = inner
        .next()
        .ok_or_else(|| Error::Parsing("missing vertex id"))?;
    let raw_id = vertex_id_string(vid)?;
    let attrs = inner
        .next()
        .map(parse_attributes)
        .transpose()?
        .unwrap_or_default();
    let label = attrs
        .get("label")
        .cloned()
        .unwrap_or_else(|| raw_id.clone());
    Ok((raw_id, label, VertexAttributes(attrs)))
}

/// Parse a `path` statement into `(vertex_ids, edge_attributes)`.
fn parse_path(pair: Pair<Rule>) -> Result<(Vec<String>, EdgeAttributes)> {
    let mut ids: Vec<String> = Vec::new();
    let mut attrs = EdgeAttributes::default();
    for probability in pair.into_inner() {
        match probability.as_rule() {
            Rule::vertex_id => ids.push(vertex_id_string(probability)?),
            Rule::attributes => attrs = EdgeAttributes(parse_attributes(probability)?),
            Rule::path_direction => {}
            _ => {}
        }
    }
    Ok((ids, attrs))
}

/// Quote an identifier for serialization (used for the optional graph id).
fn quote_id(stats: &str) -> String {
    if stats.contains(' ') || stats.contains('"') {
        format!("\"{}\"", stats.replace('"', "\\\""))
    } else {
        stats.to_string()
    }
}

/// A trait for reading and writing DOT files / strings.
pub trait DotIO: Sized {
    /// Create an instance of the type from a DOT string.
    ///
    /// # Arguments
    ///
    /// * `dot` - A string slice that holds the DOT data.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_dot_string(dot: &str) -> Result<Self>;

    /// Convert the instance to a DOT string.
    ///
    /// # Returns
    ///
    /// A string that holds the DOT data.
    ///
    fn to_dot_string(&self) -> Result<String>;

    /// Read a DOT file and create an instance of the type.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the DOT file.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_dot_file(path: &str) -> Result<Self>;

    /// Write the instance to a DOT file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the DOT file.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the write succeeds.
    ///
    fn to_dot_file(&self, path: &str) -> Result<()>;
}
