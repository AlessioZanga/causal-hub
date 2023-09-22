use std::{io::Error as IOError, path::PathBuf};

use itertools::Itertools;
use pest::{error::Error as ParserError, iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    graphs::{
        structs::{
            DirectedDenseAdjacencyMatrixGraph, PartiallyDenseAdjacencyMatrixGraph,
            UndirectedDenseAdjacencyMatrixGraph,
        },
        BaseGraph,
    },
    io::File,
    types::{FxIndexMap, FxIndexSet},
    E, L,
};

/// A GML parser.
#[derive(Clone, Debug, Default, Parser)]
#[grammar = "io/gml/grammar.pest"]
pub struct GML {
    /// The graph direction.
    pub graph_type: String,
    /// The set of vertices labels.
    pub vertices: FxIndexSet<String>,
    /// The set of edges labels.
    pub edges: Vec<(String, String)>,
}

impl<'a> From<Pair<'a, Rule>> for GML {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Assert rule match.
        assert_eq!(pair.as_rule(), Rule::graph);
        // Match inner rules.
        let mut inner = pair.into_inner();

        // Assert rule match.
        let list = inner.next().unwrap();
        assert!(matches!(list.as_rule(), Rule::list));
        // Match inner rules.
        let items = list.into_inner();

        // Initialize directionality, vertices and edges.
        let mut graph_type = "graph".to_string();
        let mut vertices = FxIndexMap::<usize, String>::default();
        let mut edges = Vec::<(usize, usize)>::new();
        // Iterate over items.
        for item in items {
            // Assert rule match.
            assert!(matches!(item.as_rule(), Rule::item));
            // Match inner rules.
            let mut inner = item.into_inner();
            // Get key and assert rule match.
            let key = inner.next().unwrap();
            assert!(matches!(key.as_rule(), Rule::key));
            // Get value.
            let value = inner.next().unwrap();

            // Match key into vertices or edges.
            match key.as_str() {
                "directed" => {
                    // Set directionality.
                    graph_type = "digraph".to_string();
                }
                "graphType" => {
                    // Set directionality for custom values.
                    graph_type = value.as_str().to_string();
                }
                "node" => {
                    // Assert rule match.
                    assert!(matches!(value.as_rule(), Rule::list));
                    // Get node attributes.
                    let attributes: FxIndexMap<_, _> = value
                        .into_inner()
                        .map(|item| {
                            // Assert rule match.
                            assert!(matches!(item.as_rule(), Rule::item));
                            // Match inner rules.
                            let mut inner = item.into_inner();
                            // Get key and assert rule match.
                            let key = inner.next().unwrap();
                            assert!(matches!(key.as_rule(), Rule::key));
                            // Get value and unquote it.
                            let value = inner.next().unwrap();

                            (key.as_str(), value.as_str().trim_matches('"'))
                        })
                        .collect();

                    // Get node id and label, if any.
                    let id = attributes["id"];
                    let label = attributes.get("label").unwrap_or(&id);

                    // TODO: Parse remaining node attributes.

                    // Add node to vertices.
                    vertices.insert(id.parse().unwrap(), label.to_string());
                }
                "edge" => {
                    // Assert rule match.
                    assert!(matches!(value.as_rule(), Rule::list));
                    // Get attributes.
                    let attributes: FxIndexMap<_, _> = value
                        .into_inner()
                        .map(|item| {
                            // Assert rule match.
                            assert!(matches!(item.as_rule(), Rule::item));
                            // Match inner rules.
                            let mut inner = item.into_inner();
                            // Get key and assert rule match.
                            let key = inner.next().unwrap();
                            assert!(matches!(key.as_rule(), Rule::key));
                            // Get value.
                            let value = inner.next().unwrap();

                            (key.as_str(), value.as_str())
                        })
                        .collect();

                    // Get source and target.
                    let (source, target) = (
                        attributes["source"].parse().unwrap(),
                        attributes["target"].parse().unwrap(),
                    );

                    // TODO: Parse remaining edge attributes.

                    // Add edge to edges.
                    edges.push((source, target));
                }
                _ => { /* TODO: Parse remaining graph attributes. */ }
            }
        }

        // Maps edges indices to labels.
        let edges = edges
            .into_iter()
            .map(|(source, target)| {
                (
                    vertices.get(&source).unwrap().clone(),
                    vertices.get(&target).unwrap().clone(),
                )
            })
            .collect();
        // Maps vertices indices to labels.
        let vertices = vertices.into_values().sorted().collect();

        Self {
            graph_type,
            vertices,
            edges,
        }
    }
}

impl From<GML> for String {
    /*
        print "graph ["
        foreach node n in g do
            print "node ["
            print "id", n.id
            (* Insert other node attributes here *)
            print "]"
        done
        foreach edge e in g do
            print "edge ["
            print "source", e.source.id
            print "target", e.target.id
            (* Insert other edge attributes here *)
            print "]"
        done
        (* Insert other graph attributes here *)
    */
    fn from(gml: GML) -> Self {
        // Initialize string.
        let mut string = String::new();

        // Print graph.
        string.push_str("graph [\n");
        // Print directionality for custom values.
        match gml.graph_type.as_str() {
            "digraph" => string.push_str("\tdirected 1\n"),
            graph_type => string.push_str(&format!("\tgraphType \"{}\"\n", graph_type)),
        }
        // Print vertices.
        for (id, label) in gml.vertices.iter().enumerate() {
            string.push_str("\tnode [\n");
            string.push_str(&format!("\t\tid {}\n", id));
            string.push_str(&format!("\t\tlabel \"{}\"\n", label));
            string.push_str("\t]\n");

            // TODO: Print remaining nodes attributes.
        }
        // Print edges.
        for (source, target) in gml.edges {
            string.push_str("\tedge [\n");
            string.push_str(&format!(
                "\t\tsource {}\n",
                gml.vertices.get_index_of(&source).unwrap()
            ));
            string.push_str(&format!(
                "\t\ttarget {}\n",
                gml.vertices.get_index_of(&target).unwrap()
            ));
            string.push_str("\t]\n");

            // TODO: Print remaining edges attributes.
        }

        // TODO: Print remaining graph attributes.

        // Print graph.
        string.push_str("]\n");

        string
    }
}

impl TryFrom<String> for GML {
    type Error = ParserError<Rule>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Parse the given string.
        let gml = Self::parse(Rule::file, string.trim())?;
        // Match inner rules.
        let gml: Self = gml.map_into().next().unwrap();

        Ok(gml)
    }
}

impl File for GML {
    type ReadError = ParserError<Rule>;

    type WriteError = IOError;

    fn read<P>(path: P) -> Result<Self, Self::ReadError>
    where
        P: Into<PathBuf>,
    {
        // Get path.
        let path = path.into();
        // Read file to string.
        let gml = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| format!("Failed to read file: \"{}\"", path.display()));
        // Parse string.
        Self::try_from(gml)
    }

    fn write<P>(self, path: P) -> Result<(), Self::WriteError>
    where
        P: Into<PathBuf>,
    {
        // Format to string.
        let string = String::from(self);
        // Write string to file.
        std::fs::write(path.into(), string)
    }
}

impl From<UndirectedDenseAdjacencyMatrixGraph> for GML {
    fn from(graph: UndirectedDenseAdjacencyMatrixGraph) -> Self {
        // Set graph type.
        let graph_type = "graph".to_string();
        // Get vertices.
        let vertices: Vec<String> = L!(graph).map_into().collect();
        // Get edges.
        let edges = E!(graph)
            .map(|(x, y)| (vertices[x].clone(), vertices[y].clone()))
            .collect();
        // Map vertices indices to labels.
        let vertices = vertices.into_iter().collect();

        Self {
            graph_type,
            vertices,
            edges,
        }
    }
}

impl From<GML> for UndirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn from(gml: GML) -> Self {
        // Assert graph type.
        assert_eq!(
            gml.graph_type, "graph",
            "GML graph type must match direction"
        );

        Self::new(gml.vertices, gml.edges)
    }
}

impl From<DirectedDenseAdjacencyMatrixGraph> for GML {
    fn from(graph: DirectedDenseAdjacencyMatrixGraph) -> Self {
        // Set graph type.
        let graph_type = "digraph".to_string();
        // Get vertices.
        let vertices: Vec<String> = L!(graph).map_into().collect();
        // Get edges.
        let edges = E!(graph)
            .map(|(x, y)| (vertices[x].clone(), vertices[y].clone()))
            .collect();
        // Map vertices indices to labels.
        let vertices = vertices.into_iter().collect();

        Self {
            graph_type,
            vertices,
            edges,
        }
    }
}

impl From<GML> for DirectedDenseAdjacencyMatrixGraph {
    #[inline]
    fn from(gml: GML) -> Self {
        // Assert graph type.
        assert_eq!(
            gml.graph_type, "digraph",
            "GML graph type must match direction"
        );

        Self::new(gml.vertices, gml.edges)
    }
}

impl From<PartiallyDenseAdjacencyMatrixGraph> for GML {
    fn from(graph: PartiallyDenseAdjacencyMatrixGraph) -> Self {
        // Set directionality.
        let graph_type = "pdgraph".to_string();
        // Get vertices.
        let vertices: Vec<String> = L!(graph).map_into().collect();
        // Get edges.
        let edges = E!(graph)
            .map(|(x, y)| (vertices[x].clone(), vertices[y].clone()))
            .collect();
        // Map vertices indices to labels.
        let vertices = vertices.into_iter().collect();

        Self {
            graph_type,
            vertices,
            edges,
        }
    }
}
