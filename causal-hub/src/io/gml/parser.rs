#![allow(missing_docs)]

use std::collections::HashMap;

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::types::{Error, Result, Set};

/// A GML parser built on top of `pest`.
///
/// The parsed representation is graph-agnostic: it stores the graph direction
/// (`graph_type`), the (sorted) vertex labels, and the list of directed edges
/// as label pairs. Converting to a concrete graph type (`DiGraph` / `UnGraph`)
/// validates the directionality.
#[allow(missing_docs)]
#[derive(Parser)]
#[grammar = "src/io/gml/grammar.pest"]
pub struct GMLParser;

/// A graph-agnostic GML representation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GML {
    /// The graph direction (`"graph"` for undirected, `"digraph"` for directed).
    pub graph_type: String,
    /// The set of vertex labels (sorted).
    pub vertices: Set<String>,
    /// The list of directed edges as label pairs.
    pub edges: Vec<(String, String)>,
}

impl GML {
    /// Parse a GML string into the graph-agnostic representation.
    ///
    /// # Arguments
    ///
    /// * `string` - The GML content.
    ///
    /// # Returns
    ///
    /// The parsed [`GML`] representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not valid GML.
    ///
    pub fn from_string(string: &str) -> Result<Self> {
        let mut pairs = GMLParser::parse(Rule::file, string.trim())
            .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
        let pair = pairs
            .next()
            .ok_or_else(|| Error::Parsing("empty GML document"))?;
        Self::from_pair(pair)
    }

    /// Build a [`GML`] from a parsed `graph` pair.
    fn from_pair(pair: Pair<Rule>) -> Result<Self> {
        if pair.as_rule() != Rule::graph {
            return Err(Error::Parsing("expected a GML graph"));
        }

        let mut inner = pair.into_inner();
        let list = inner
            .next()
            .ok_or_else(|| Error::Parsing("missing graph list"))?;
        if list.as_rule() != Rule::list {
            return Err(Error::Parsing("expected a GML list"));
        }

        let mut graph_type = "graph".to_string();
        let mut vertices_map: HashMap<usize, String> = HashMap::new();
        let mut edges: Vec<(usize, usize)> = Vec::new();

        for item in list.into_inner() {
            if item.as_rule() != Rule::item {
                return Err(Error::Parsing("expected a GML item"));
            }
            let mut inner = item.into_inner();
            let key = inner
                .next()
                .ok_or_else(|| Error::Parsing("missing item key"))?;
            if key.as_rule() != Rule::key {
                return Err(Error::Parsing("expected a GML key"));
            }
            let value = inner
                .next()
                .ok_or_else(|| Error::Parsing("missing item value"))?;

            match key.as_str() {
                "directed" => {
                    graph_type = "digraph".to_string();
                }
                "graphType" => {
                    graph_type = value.as_str().trim_matches('"').to_string();
                }
                "node" => {
                    if value.as_rule() != Rule::list {
                        return Err(Error::Parsing("node must be a list"));
                    }
                    let mut id: Option<usize> = None;
                    let mut label: Option<String> = None;
                    for attr in value.into_inner() {
                        if attr.as_rule() != Rule::item {
                            continue;
                        }
                        let mut ai = attr.into_inner();
                        let k = ai
                            .next()
                            .ok_or_else(|| Error::Parsing("missing attribute key"))?;
                        let v = ai
                            .next()
                            .ok_or_else(|| Error::Parsing("missing attribute value"))?;
                        match k.as_str() {
                            "id" => {
                                id = Some(
                                    v.as_str()
                                        .trim()
                                        .parse()
                                        .map_err(|_| Error::Parsing("invalid node id"))?,
                                )
                            }
                            "label" => {
                                label = Some(v.as_str().trim_matches('"').to_string());
                            }
                            _ => {}
                        }
                    }
                    let id = id.ok_or_else(|| Error::Parsing("node without id"))?;
                    let label = label.unwrap_or_else(|| id.to_string());
                    vertices_map.insert(id, label);
                }
                "edge" => {
                    if value.as_rule() != Rule::list {
                        return Err(Error::Parsing("edge must be a list"));
                    }
                    let mut source: Option<usize> = None;
                    let mut target: Option<usize> = None;
                    for attr in value.into_inner() {
                        if attr.as_rule() != Rule::item {
                            continue;
                        }
                        let mut ai = attr.into_inner();
                        let k = ai
                            .next()
                            .ok_or_else(|| Error::Parsing("missing attribute key"))?;
                        let v = ai
                            .next()
                            .ok_or_else(|| Error::Parsing("missing attribute value"))?;
                        match k.as_str() {
                            "source" => {
                                source = Some(
                                    v.as_str()
                                        .trim()
                                        .parse()
                                        .map_err(|_| Error::Parsing("invalid edge source"))?,
                                )
                            }
                            "target" => {
                                target = Some(
                                    v.as_str()
                                        .trim()
                                        .parse()
                                        .map_err(|_| Error::Parsing("invalid edge target"))?,
                                )
                            }
                            _ => {}
                        }
                    }
                    let source = source.ok_or_else(|| Error::Parsing("edge without source"))?;
                    let target = target.ok_or_else(|| Error::Parsing("edge without target"))?;
                    edges.push((source, target));
                }
                _ => {}
            }
        }

        // Resolve edge endpoints to labels.
        let edges: Vec<(String, String)> = edges
            .into_iter()
            .map(|(stats, t)| {
                let stats = vertices_map
                    .get(&stats)
                    .cloned()
                    .ok_or_else(|| Error::Parsing("edge references unknown node"))?;
                let t = vertices_map
                    .get(&t)
                    .cloned()
                    .ok_or_else(|| Error::Parsing("edge references unknown node"))?;
                Ok((stats, t))
            })
            .collect::<Result<Vec<_>>>()?;

        // Collect and sort the vertex labels for deterministic output.
        let mut vlist: Vec<String> = vertices_map.into_values().collect();
        vlist.sort();
        let vertices: Set<String> = Set::from_iter(vlist);

        Ok(Self {
            graph_type,
            vertices,
            edges,
        })
    }
}

/// Serialize a [`GML`] representation into a GML string.
pub(crate) fn serialize(gml: &GML) -> Result<String> {
    let mut string = String::new();
    string.push_str("graph [\n");

    // Print directionality.
    match gml.graph_type.as_ref() {
        "digraph" => string.push_str("\tdirected 1\n"),
        graph_type => string.push_str(&format!("\tgraphType \"{}\"\n", graph_type)),
    }

    // Print vertices (id is the sorted position).
    for (id, label) in gml.vertices.iter().enumerate() {
        string.push_str("\tnode [\n");
        string.push_str(&format!("\t\tid {}\n", id));
        string.push_str(&format!("\t\tlabel \"{}\"\n", label));
        string.push_str("\t]\n");
    }

    // Print edges.
    for (source, target) in &gml.edges {
        let sid = gml
            .vertices
            .get_index_of(source)
            .ok_or_else(|| Error::Parsing("edge references unknown node"))?;
        let tid = gml
            .vertices
            .get_index_of(target)
            .ok_or_else(|| Error::Parsing("edge references unknown node"))?;
        string.push_str("\tedge [\n");
        string.push_str(&format!("\t\tsource {}\n", sid));
        string.push_str(&format!("\t\ttarget {}\n", tid));
        string.push_str("\t]\n");
    }

    string.push_str("]\n");
    Ok(string)
}

/// A trait for reading and writing GML files / strings.
pub trait GmlIO: Sized {
    /// Create an instance of the type from a GML string.
    ///
    /// # Arguments
    ///
    /// * `gml` - A string slice that holds the GML data.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_gml_string(gml: &str) -> Result<Self>;

    /// Convert the instance to a GML string.
    ///
    /// # Returns
    ///
    /// A string that holds the GML data.
    ///
    fn to_gml_string(&self) -> Result<String>;

    /// Read a GML file and create an instance of the type.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the GML file.
    ///
    /// # Returns
    ///
    /// A new instance of the type.
    ///
    fn from_gml_file(path: &str) -> Result<Self>;

    /// Write the instance to a GML file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the GML file.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the write succeeds.
    ///
    fn to_gml_file(&self, path: &str) -> Result<()>;
}
