mod attributes;
pub use attributes::*;

mod parser;
use std::sync::Arc;

pub use parser::{DOT, DOTParser, DotIO};

use crate::{
    models::{DiGraph, Graph, HasLabels, UnGraph},
    types::{Error, Result},
};

/// Convert a directed graph into the graph-agnostic DOT representation.
impl From<&DiGraph> for DOT {
    fn from(graph: &DiGraph) -> Self {
        let labels: Vec<&String> = graph.labels().iter().collect();
        let vertices = graph
            .labels()
            .iter()
            .map(|l| (l.clone(), VertexAttributes::default()))
            .collect();
        let edges = graph
            .edges()
            .iter()
            .map(|&(x, y)| {
                (
                    labels[x].clone(),
                    labels[y].clone(),
                    EdgeAttributes::default(),
                )
            })
            .collect();
        Self {
            graph_type: "digraph".to_string(),
            strict: false,
            id: None,
            graph_attributes: GraphAttributes::default(),
            default_node_attributes: VertexAttributes::default(),
            default_edge_attributes: EdgeAttributes::default(),
            vertices,
            edges,
        }
    }
}

/// Build a directed graph from the graph-agnostic DOT representation.
impl TryFrom<DOT> for DiGraph {
    type Error = Error;

    fn try_from(dot: DOT) -> Result<Self> {
        if dot.graph_type != "digraph" {
            return Err(Error::InvalidParameter(
                "dot graph type",
                "expected 'digraph' for a directed graph",
            ));
        }
        let labels: Vec<String> = dot.vertices.keys().cloned().collect();
        let mut graph = DiGraph::empty(labels)?;
        for (stats, t, _) in dot.edges {
            let x = graph.label_to_index(&stats)?;
            let y = graph.label_to_index(&t)?;
            graph.add_edge(x, y)?;
        }
        Ok(graph)
    }
}

/// Convert an undirected graph into the graph-agnostic DOT representation.
impl From<&UnGraph> for DOT {
    fn from(graph: &UnGraph) -> Self {
        let labels: Vec<&String> = graph.labels().iter().collect();
        let vertices = graph
            .labels()
            .iter()
            .map(|l| (l.clone(), VertexAttributes::default()))
            .collect();
        let edges = graph
            .edges()
            .iter()
            .map(|&(x, y)| {
                (
                    labels[x].clone(),
                    labels[y].clone(),
                    EdgeAttributes::default(),
                )
            })
            .collect();
        Self {
            graph_type: "graph".to_string(),
            strict: false,
            id: None,
            graph_attributes: GraphAttributes::default(),
            default_node_attributes: VertexAttributes::default(),
            default_edge_attributes: EdgeAttributes::default(),
            vertices,
            edges,
        }
    }
}

/// Build an undirected graph from the graph-agnostic DOT representation.
impl TryFrom<DOT> for UnGraph {
    type Error = Error;

    fn try_from(dot: DOT) -> Result<Self> {
        if dot.graph_type != "graph" {
            return Err(Error::InvalidParameter(
                "dot graph type",
                "expected 'graph' for an undirected graph",
            ));
        }
        let labels: Vec<String> = dot.vertices.keys().cloned().collect();
        let mut graph = UnGraph::empty(labels)?;
        for (stats, t, _) in dot.edges {
            let x = graph.label_to_index(&stats)?;
            let y = graph.label_to_index(&t)?;
            graph.add_edge(x, y)?;
        }
        Ok(graph)
    }
}

impl DotIO for DiGraph {
    fn from_dot_string(dot: &str) -> Result<Self> {
        DiGraph::try_from(DOT::from_string(dot)?)
    }

    fn to_dot_string(&self) -> Result<String> {
        DOT::from(self).to_string_repr()
    }

    fn from_dot_file(path: &str) -> Result<Self> {
        let string =
            std::fs::read_to_string(path).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Self::from_dot_string(&string)
    }

    fn to_dot_file(&self, path: &str) -> Result<()> {
        let string = self.to_dot_string()?;
        std::fs::write(path, string).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Ok(())
    }
}

impl DotIO for UnGraph {
    fn from_dot_string(dot: &str) -> Result<Self> {
        UnGraph::try_from(DOT::from_string(dot)?)
    }

    fn to_dot_string(&self) -> Result<String> {
        DOT::from(self).to_string_repr()
    }

    fn from_dot_file(path: &str) -> Result<Self> {
        let string =
            std::fs::read_to_string(path).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Self::from_dot_string(&string)
    }

    fn to_dot_file(&self, path: &str) -> Result<()> {
        let string = self.to_dot_string()?;
        std::fs::write(path, string).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Ok(())
    }
}
